use super::page;
use super::page::{PageIndex, SliceIndex};
use super::traits::PagesStorage;
use std::collections::HashMap;

/// Defines a page-slice memory layout.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PageSliceLayout {
    /// The slice index
    pub slice_idx: SliceIndex,

    /// The page index the slices belong to
    pub page_idx: PageIndex,

    /// The page offset where the slice starts
    pub offset: u32,

    /// The length of the slice in bytes
    pub len: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct PageSlice {
    dirty: bool,
    data: Vec<u8>,
    layout: PageSliceLayout,
}

#[derive(Debug, Clone, PartialEq)]
enum CachedPageSlice {
    // We didn't load the page-slice yet from the underlying db
    NotCached,

    // We've loaded page-slice from the underlying db, but no data was there
    CachedEmpty,

    // We've loaded the page-slice from the underlying db and it had data
    Cached(PageSlice),
}

/// `PageSliceCache` is a caching layer on top of the `PageCache`.
/// While `PageCache` deals with data involving only page units, `PageSliceCache` has fine-grained
/// control for various sized of data.
pub struct PageSliceCache<'pc, PC> {
    // The `ith item` will say whether the `ith page slice` is dirty
    cached_slices: Vec<CachedPageSlice>,

    page_cache: &'pc mut PC,
}

impl<'pc, PC: PagesStorage> PageSliceCache<'pc, PC> {
    /// Initializes a new `PageSliceCache` instance.
    ///
    /// * `page_cache` - an instance of `PageCache` in charge of supplying the persisted pages
    ///  upon requests (`read_page`) and for propagating new pages versions (`write_page`).
    ///  However, persistence only takes place by triggerring `commit`
    ///
    /// * `max_page_slices` - the maximum number of page-slices the `PageSliceCache` instance could
    ///   use when doing read / write. A page slice index is within the range `0..(max_page_slices - 1)` (inclusive)
    pub fn new(page_cache: &'pc mut PC, max_page_slices: usize) -> Self {
        Self {
            page_cache,
            cached_slices: vec![CachedPageSlice::NotCached; max_page_slices],
        }
    }

    /// * We check against `cached_slices` whether we already have the requsted page-slice
    ///
    /// * If we do, we return it. otherwise, we call `page_cache.read_page()` for the page
    ///   of the given slice and cache the page slice for future use under `cached_slices`
    ///
    /// * In case the page slice is empty we cache it as `CachedEmpty`.
    ///   Else, we cache it as `Cached(PageSlice)` and mark the page-slice as non-dirty
    pub fn read_page_slice(&mut self, layout: &PageSliceLayout) -> Option<Vec<u8>> {
        let slice_index = layout.slice_idx.0 as usize;

        assert!(slice_index < self.cached_slices.len());

        let slice = &self.cached_slices[slice_index];

        match slice {
            CachedPageSlice::NotCached => {
                // page-slice isn't cached, so we first need to load the underlying page from
                // `page_cache`

                let page: Option<Vec<u8>> = self.page_cache.read_page(layout.page_idx.0);

                if page.is_some() {
                    // `page` has been fetched from the `page cache`
                    let data = page.unwrap();

                    let slice = PageSlice {
                        layout: layout.clone(),
                        dirty: false,
                        data: data.clone(),
                    };

                    std::mem::replace(
                        &mut self.cached_slices[slice_index],
                        CachedPageSlice::Cached(slice),
                    );

                    let start = layout.offset as usize;
                    let end = (layout.offset + layout.len) as usize;
                    let slice_data = data[start..end].to_vec();

                    Some(slice_data)
                } else {
                    // `page` is a `None`. That means there is no real data storaged for this page right now.
                    // Therefore, there is no data stored any page-slice too.

                    let slice = PageSlice {
                        layout: layout.clone(),
                        dirty: false,
                        data: Vec::new(),
                    };

                    std::mem::replace(
                        &mut self.cached_slices[slice_index],
                        CachedPageSlice::Cached(slice),
                    );

                    None
                }
            }
            CachedPageSlice::CachedEmpty => None,
            CachedPageSlice::Cached(slice) => Some(slice.data.to_vec()),
        }
    }

    /// * We insert the new page slice into `cached_slices` as `Cached(PageSlice)` and mark it as dirty
    pub fn write_page_slice(&mut self, layout: &PageSliceLayout, data: Vec<u8>) {
        // We don't mind whether the underlying page is already in the cache or not.
        // We just save the new written page-slice and mark it as `dirty`.

        let slice_index = layout.slice_idx.0 as usize;

        assert!(slice_index < self.cached_slices.len());

        let slice = PageSlice {
            layout: layout.clone(),
            dirty: true,
            data,
        };

        std::mem::replace(
            &mut self.cached_slices[slice_index],
            CachedPageSlice::Cached(slice),
        );
    }

    /// * Clears the `cached_slices`
    /// * Clears the `page_cache`, by calling `page_cache.clear()`
    #[doc(hidden)]
    #[cfg(test)]
    pub fn clear(&mut self) {
        let max_page_slices = self.cached_slices.len();

        self.cached_slices = vec![CachedPageSlice::NotCached; max_page_slices];
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
    /// * We do `page_cache.commit()`
    ///
    /// * We don't do a `clear`. In real-life usage, the `svm` will call a `commit()`
    ///   after terimnating execution of the smart contract. The `clear` method is intended to be
    ///   used *only* for `tests`
    pub fn commit(&mut self) {
        let mut page_slices = HashMap::<u32, Vec<PageSlice>>::new();
        let mut pages_indexes = Vec::<u32>::new();

        for cs in &self.cached_slices {
            if let CachedPageSlice::Cached(ref slice) = cs {
                if slice.dirty {
                    let page_idx = slice.layout.page_idx.0;

                    let entry: &mut Vec<_> = page_slices.entry(page_idx).or_insert(Vec::new());
                    entry.push(slice.clone());

                    pages_indexes.push(page_idx);
                }
            }
        }

        pages_indexes.dedup();

        let mut pages = pages_indexes
            .iter()
            .map(|&page_idx| {
                let page_bytes = if let Some(bytes) = self.page_cache.read_page(page_idx) {
                    bytes
                } else {
                    page::zero_page()
                };

                (page_idx, page_bytes)
            })
            .collect::<HashMap<u32, Vec<u8>>>();

        for (page_idx, slices) in page_slices {
            let page = pages.get_mut(&page_idx).unwrap();

            for slice in slices {
                self.patch_page(page, slice);
            }
        }

        // propagating the new versioned pages to `page_cache`
        for (page_idx, page) in pages {
            self.page_cache.write_page(page_idx, &page);
        }

        self.page_cache.commit();
    }

    /// Applies `slice` on top of `page`
    fn patch_page(&self, page: &mut Vec<u8>, slice: PageSlice) {
        let start = slice.layout.offset as usize;
        let end = (slice.layout.offset + slice.layout.len) as usize;

        let dst_slice = &mut page[start..end];

        dst_slice.copy_from_slice(&slice.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::KVStore;
    use crate::{MemPages, PageCache};

    pub type MemPageCache<'pc, K = [u8; 32]> = PageCache<'pc, MemPages<K>>;

    macro_rules! page_hash {
        ($addr: expr, $page_idx: expr) => {{
            use crate::traits::PageHasher;
            use crate::DefaultPageHasher;

            let addr = Address::from($addr as u32);

            DefaultPageHasher::hash(addr, $page_idx)
        }};
    }

    macro_rules! setup_cache {
        ($page_slice_cache: ident, $db: ident, $addr: expr, $max_pages: expr, $max_page_slices: expr) => {
            use crate::MemKVStore;
            use std::cell::RefCell;
            use std::rc::Rc;
            use svm_common::Address;

            let addr = Address::from($addr as u32);

            let $db = Rc::new(RefCell::new(MemKVStore::new()));
            let db_clone = Rc::clone(&$db);

            let mut inner = MemPages::new(addr, db_clone);

            let mut page_cache = MemPageCache::new(&mut inner, $max_pages);

            let mut $page_slice_cache = PageSliceCache::new(&mut page_cache, $max_page_slices);
        };
    }

    #[test]
    fn loading_an_empty_slice_into_the_cache() {
        setup_cache!(cache, db, 0x11_22_33_44, 10, 100);

        let layout = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 200,
        };

        assert_eq!(None, cache.read_page_slice(&layout));
    }

    #[test]
    fn read_an_empty_slice_then_override_it_and_then_commit() {
        setup_cache!(cache, db, 0x11_22_33_44, 10, 100);

        let layout = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 3,
        };

        assert_eq!(None, cache.read_page_slice(&layout));

        cache.write_page_slice(&layout, vec![10, 20, 30]);

        assert_eq!(Some(vec![10, 20, 30]), cache.read_page_slice(&layout));

        // page is not persisted though since we didn't `commit`
        let ph = page_hash!(0x11_22_33_44, 0);
        assert_eq!(None, db.borrow().get(ph));
    }

    #[test]
    fn write_slice_without_loading_it_first_and_commit() {
        setup_cache!(cache, db, 0x11_22_33_44, 10, 100);

        let layout = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 3,
        };

        cache.write_page_slice(&layout, vec![10, 20, 30]);
        cache.commit();

        let ph = page_hash!(0x11_22_33_44, 1);

        assert_eq!(Some(vec![10, 20, 30]), cache.read_page_slice(&layout));

        let page = db.borrow().get(ph).unwrap();
        assert_eq!(vec![10, 20, 30], &page[100..103]);
    }

    #[test]
    fn read_an_existing_slice_then_overriding_it_and_commit() {
        setup_cache!(cache, db, 0x11_22_33_44, 10, 100);

        let layout = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 3,
        };

        cache.write_page_slice(&layout, vec![10, 20, 30]);
        cache.commit();

        let ph = page_hash!(0x11_22_33_44, 1);

        let page = db.borrow().get(ph).unwrap();
        assert_eq!(vec![10, 20, 30], &page[100..103]);

        cache.write_page_slice(&layout, vec![40, 50, 60]);

        // new page is on the page-cache, but not persisted yet
        assert_eq!(Some(vec![40, 50, 60]), cache.read_page_slice(&layout));
        let page = db.borrow().get(ph).unwrap();
        assert_eq!(vec![10, 20, 30], &page[100..103]);

        // now we also persist the new page version
        cache.commit();

        let page = db.borrow().get(ph).unwrap();
        assert_eq!(vec![40, 50, 60], &page[100..103]);
    }

    #[test]
    fn write_slice_and_commit_then_load_it_override_it_and_commit() {
        setup_cache!(cache, db, 0x11_22_33_44, 10, 100);

        let layout = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 3,
        };

        let ph = page_hash!(0x11_22_33_44, 1);

        // 1) first page write
        cache.write_page_slice(&layout, vec![10, 20, 30]);

        // 2) commit
        cache.commit();

        // 3) load persisted page (we do a `clear` first to make sure we load from storage)
        cache.clear();

        assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout).unwrap());

        // 4) page override
        cache.write_page_slice(&layout, vec![40, 50, 60]);
        assert_eq!(vec![40, 50, 60], cache.read_page_slice(&layout).unwrap());

        // 5) commit again
        let page = db.borrow().get(ph).unwrap();
        assert_eq!(vec![10, 20, 30], &page[100..103]);

        cache.commit();

        let page = db.borrow().get(ph).unwrap();
        assert_eq!(vec![40, 50, 60], &page[100..103]);
    }

    #[test]
    fn write_two_slices_under_same_page_and_commit() {
        setup_cache!(cache, db, 0x11_22_33_44, 10, 100);

        let layout1 = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 3,
        };

        let layout2 = PageSliceLayout {
            slice_idx: SliceIndex(1),
            page_idx: PageIndex(1),
            offset: 200,
            len: 2,
        };

        let ph = page_hash!(0x11_22_33_44, 1);

        cache.write_page_slice(&layout1, vec![10, 20, 30]);
        cache.write_page_slice(&layout2, vec![40, 50]);

        assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout1).unwrap());
        assert_eq!(vec![40, 50], cache.read_page_slice(&layout2).unwrap());

        // commiting two slices under the same page
        assert_eq!(None, db.borrow().get(ph));

        cache.commit();
        cache.clear();

        let page = db.borrow().get(ph).unwrap();
        assert_eq!(vec![10, 20, 30], &page[100..103]);
        assert_eq!(vec![40, 50], &page[200..202]);

        assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout1).unwrap());
        assert_eq!(vec![40, 50], cache.read_page_slice(&layout2).unwrap());
    }
}
