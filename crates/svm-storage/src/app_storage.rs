use crate::{
    page::{self, PageIndex, PageOffset, PageSliceLayout},
    traits::PageCache,
};

use std::collections::HashMap;

use svm_abi::{query::StorageReader, schema::VarLayout};
use svm_common::State;

use log::{debug, trace};

#[derive(Debug, Clone, PartialEq)]
struct PageSlice {
    dirty: bool,
    data: Vec<u8>,
    layout: PageSliceLayout,
}

/// `AppStorage` is a caching layer on top of the `PageCache`.
/// While `PageCache` deals with data involving only page units, `AppStorage` has fine-grained
/// control for various sized of data.
pub struct AppStorage {
    cached_slices: HashMap<PageIndex, HashMap<PageOffset, PageSlice>>,

    page_cache: Box<dyn PageCache>,
}

impl AppStorage {
    /// Initializes a new `AppStorage` instance.
    ///
    /// * `page_cache` - implements the `PageCache` trait. In charge of supplying the pages
    ///  upon requests (`read_page`) and for propagating new pages versions (`write_page`).
    ///  However, persistence only takes place by triggerring `commit`
    pub fn new(page_cache: Box<dyn PageCache>) -> Self {
        Self {
            page_cache,
            cached_slices: HashMap::new(),
        }
    }

    /// * We check against `cached_slices` whether we already have the requsted page-slice
    ///
    /// * If we do, we return it. otherwise, we call `page_cache.read_page()` for the page
    ///   of the given slice and cache the page slice for future use under `cached_slices`
    ///
    /// * In case the page slice is empty we cache it as `CachedEmpty`.
    ///   Else, we cache it as `Cached(PageSlice)` and mark the page-slice as non-dirty
    pub fn read_page_slice(&mut self, layout: &PageSliceLayout) -> Vec<u8> {
        debug!("reading page-slice: {:?}", layout);

        let page_idx = layout.page_index();

        match self.get_page_slices(page_idx) {
            None => {
                // there are no page-slices loaded yet
                self.do_init_page_slices(page_idx);
            }
            Some(pslices) => {
                let slice = self.do_read_page_slice(pslices, layout.page_offset(), layout.len());

                match slice {
                    None => {
                        // page-slice isn't in the cache
                    }
                    Some(slice) => {
                        trace!("cache hit for page-slice {:?}", layout);

                        // page-slice is cached already, so we're left with returning a clone of its `data`
                        let bytes = slice.data.clone();
                        return bytes;
                    }
                }
            }
        }

        // the page-slice isn't cached, so we first need to load the underlying page from `page_cache`
        trace!("cache miss for page-slice: {:?}", layout);
        let page = self.page_cache.read_page(layout.page_index());
        let page: Option<&Vec<u8>> = page.as_ref();

        let slice_data = match page {
            Some(page) => {
                let start = layout.page_offset().0 as usize;
                let end = (layout.page_offset().0 + layout.len()) as usize;
                page[start..end].to_vec()
            }
            None => {
                // There is no real data stored for this page right now.
                // Therefore, there is no data stored under any page-slice too.
                // We represent that page-slice as a zeros-slice.
                vec![0; layout.len() as usize]
            }
        };

        let slice = PageSlice {
            layout: layout.clone(),
            dirty: false,
            data: slice_data.clone(),
        };

        // cache the slice for the future
        let page_slices = self.get_page_slices_mut(layout.page_index()).unwrap();
        page_slices.insert(layout.page_offset(), slice);

        slice_data
    }

    /// Insert the new page slice into `cached_slices` and mark it as dirty
    pub fn write_page_slice(&mut self, layout: &PageSliceLayout, data: &[u8]) {
        // We don't mind whether the underlying page is already in the cache or not.
        // We just save the new written page-slice and mark it as `dirty`.
        debug!("writing page-slice (not persisting yet) {:?}", layout);
        trace!("    page-slice data:");
        trace!("    {:?}", data);

        assert!(layout.len() < page::PAGE_SIZE);

        let page_idx = layout.page_index();

        if self.get_page_slices(page_idx).is_none() {
            self.do_init_page_slices(page_idx);
        }

        let slice = PageSlice {
            layout: layout.clone(),
            dirty: true,
            data: data[0..layout.len() as usize].to_vec(),
        };

        let page_slices = self.get_page_slices_mut(page_idx).unwrap();

        page_slices.insert(layout.page_offset(), slice);
    }

    /// * Clears the `cached_slices`
    /// * Clears the `page_cache`, by calling `page_cache.clear()`
    #[doc(hidden)]
    #[cfg(test)]
    pub fn clear(&mut self) {
        debug!("clearing page-slice cache...");

        self.cached_slices.clear();
        self.page_cache.clear();
    }

    /// * Scans for dirty pages-slices and groupc them by `page index`
    ///
    /// * We say that a page is dirty if it has least one page-slice
    ///
    /// * For each dirty page we read its old page data (by calling `page_cache.read_page(..)`)
    ///   and then we play its new page-slices changes on top of it
    ///
    /// * For each patched cached page we do `page_cache.write_page(..)`
    ///
    /// * We do `page_cache.commit()` and return the new underlying page-storage `State`.
    ///
    /// * We don't do a `clear`. In real-life usage, the `svm` will call a `commit()`
    ///   after terimnating execution of the app. The `clear` method is intended to be
    ///   used *only* for `tests`
    #[must_use]
    pub fn commit(&mut self) -> State {
        debug!("commiting page-slice cache to underlying pages-storage");

        let mut dirty_pages_slices = HashMap::<PageIndex, Vec<PageSlice>>::new();
        let mut dirty_pages_indexes = Vec::<PageIndex>::new();

        for page_slices in self.cached_slices.values() {
            for slice in page_slices.values() {
                if slice.dirty {
                    let page_idx = slice.layout.page_index();

                    let entry: &mut Vec<_> =
                        dirty_pages_slices.entry(page_idx).or_insert_with(Vec::new);
                    entry.push(slice.clone());

                    dirty_pages_indexes.push(page_idx);
                }
            }
        }

        let mut dirty_pages = dirty_pages_indexes
            .iter()
            .map(|&page_idx| {
                let page_bytes = if let Some(bytes) = self.page_cache.read_page(page_idx) {
                    bytes
                } else {
                    page::zero_page()
                };

                (page_idx, page_bytes)
            })
            .collect::<HashMap<PageIndex, Vec<u8>>>();

        for (page_idx, dirty_slices) in dirty_pages_slices {
            let dirty_page = dirty_pages.get_mut(&page_idx).unwrap();

            for slice in dirty_slices {
                self.patch_page(dirty_page, slice);
            }
        }

        // propagating the new versioned pages to `page_cache`
        for (page_idx, page) in dirty_pages {
            self.page_cache.write_page(page_idx, &page);
        }

        self.page_cache.commit();
        debug!("finished commiting page-slice cache changes...");

        let state = self.page_cache.get_state();
        debug!("new app state: {:?}", state);

        state
    }

    /// Applies a slice edit on top of a `page`
    fn patch_page(&self, page: &mut Vec<u8>, slice: PageSlice) {
        let start = slice.layout.page_offset().0 as usize;
        let end = (slice.layout.page_offset().0 + slice.layout.len()) as usize;

        let data = &mut page[start..end];
        data.copy_from_slice(&slice.data);
    }

    #[inline]
    fn get_page_slices(&self, page_idx: PageIndex) -> Option<&HashMap<PageOffset, PageSlice>> {
        self.cached_slices.get(&page_idx)
    }

    #[inline]
    fn get_page_slices_mut(
        &mut self,
        page_idx: PageIndex,
    ) -> Option<&mut HashMap<PageOffset, PageSlice>> {
        self.cached_slices.get_mut(&page_idx)
    }

    fn do_read_page_slice<'a>(
        &self,
        page_slices: &'a HashMap<PageOffset, PageSlice>,
        offset: PageOffset,
        len: u32,
    ) -> Option<&'a PageSlice> {
        let slice = page_slices.get(&offset);

        if let Some(inner) = slice {
            debug_assert_eq!(offset, inner.layout.page_offset());
            assert_eq!(len, inner.layout.len());
        }

        slice
    }

    #[inline]
    fn do_init_page_slices(&mut self, page_idx: PageIndex) {
        self.cached_slices.insert(page_idx, HashMap::new());
    }
}

impl StorageReader for AppStorage {
    fn read_raw_var(&mut self, layout: &VarLayout) -> Option<Vec<u8>> {
        let layout: PageSliceLayout = layout.into();
        let bytes = self.read_page_slice(&layout);

        Some(bytes)
    }
}

impl Drop for AppStorage {
    fn drop(&mut self) {
        debug!("dropping `AppStorage`...");
    }
}

impl std::fmt::Debug for AppStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "[DEBUG] PageCacheSlice")?;
        writeln!(f, "#Allocated slices: {}", self.cached_slices.len())?;

        for page_slices in self.cached_slices.values() {
            for slice in page_slices.values() {
                writeln!(
                    f,
                    "#({}, {}, {})",
                    slice.layout.page_index().0,
                    slice.layout.page_offset().0,
                    slice.layout.len(),
                )?;
                writeln!(f, "   dirty: {}", slice.dirty)?;
                writeln!(f, "   data: {:?}", slice.data)?;
            }
        }

        writeln!(f)
    }
}
