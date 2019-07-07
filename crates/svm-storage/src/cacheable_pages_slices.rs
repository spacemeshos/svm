use super::traits::PagesStorage;
use std::collections::HashMap;

/// `PageSliceLayout` is a tuple representing the page-slice layout as: `(index, page_idx, offset, len)`
/// * `index`    - the slice index
/// * `page_idx` - the page index
/// * `offset`   - the page offset where the slice starts
/// * `len`      - the length of the slice
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PageSliceLayout(u32, u32, u32, u32);

#[derive(Debug, Clone, PartialEq)]
pub struct PageSlice {
    dirty: bool,
    data: Vec<u8>,
    layout: PageSliceLayout,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CachedPageSlice {
    // We didn't load the page-slice yet from the underlying db
    NotCached,

    // We've loaded page-slice from the underlying db, but no data was there
    CachedEmpty,

    // We've loaded the page-slice from the underlying db and it had data
    Cached(PageSlice),
}

pub struct CacheablePagesSlices<'ps, PS> {
    // The `ith item` will say whether the `ith page slice` is dirty
    cached_slices: Vec<CachedPageSlice>,

    cached_pages: &'ps mut PS,
}

impl<'ps, PS: PagesStorage> CacheablePagesSlices<'ps, PS> {
    pub fn new(cached_pages: &'ps mut PS, max_page_slices: usize) -> Self {
        Self {
            cached_pages,
            cached_slices: vec![CachedPageSlice::NotCached; max_page_slices],
        }
    }

    /// * We check against `cached_slices` whether we already have the requsted page-slice
    ///
    /// * If we do, we return it. otherwise, we call `cached_pages.read_page()` for the page
    ///   of the given slice and cache the page slice for future use under `cached_slices`
    ///
    /// * In case the page slice is empty we cache it as `CachedEmpty`.
    ///   Else, we cache it as `Cached(PageSlice)` and mark the page-slice as non-dirty
    pub fn read_page_slice(&mut self, layout: PageSliceLayout) -> Option<Vec<u8>> {
        let slice_index = layout.0 as usize;

        assert!(slice_index < self.cached_slices.len());

        let slice = &self.cached_slices[slice_index];

        match slice {
            CachedPageSlice::NotCached => {
                // page-slice isn't cached, so we first need to load the underlying page from `cached_pages`.

                let page_idx = layout.1;
                let page: Option<Vec<u8>> = self.cached_pages.read_page(page_idx);

                if page.is_some() {
                    None
                } else {
                    // `page` is a `None`. That means there is no real data storaged for this page right now.
                    // Therefore, there is no data stored any page-slice too.

                    let slice = PageSlice {
                        layout: layout,
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
    pub fn write_page_slice(&mut self, layout: PageSliceLayout, data: Vec<u8>) {
        // We don't mind whether the underlying page is already in the cache or not.
        // We just save the new written page-slice and mark it as `dirty`.

        let slice_index = layout.0 as usize;

        assert!(slice_index < self.cached_slices.len());

        let slice = PageSlice {
            layout: layout,
            dirty: true,
            data,
        };

        std::mem::replace(
            &mut self.cached_slices[slice_index],
            CachedPageSlice::Cached(slice),
        );
    }

    /// * Clears the `cached_slices`
    /// * Clears the `cached_pages`, by calling `cached_pages.clear()`
    pub fn clear(&mut self) {
        self.cached_slices.clear();
        self.cached_pages.clear();
    }

    /// * Scans for dirty pages-slices and groups them by `page index`
    ///
    /// * We say that a page is dirty if it has least one page-slice
    ///
    /// * For each dirty page we read its old page data (by calling `cached_pages.read_page(..)`)
    ///   and then we play its new page-slices changes on top of it
    ///
    /// * For each patched cached page we do `cached_pages.write_page(..)`
    ///
    /// * We do `cached_pages.commit()`
    ///
    /// * We call `clear()` in order to reset the state
    pub fn commit(&mut self) {
        let mut page_slices = HashMap::<u32, Vec<PageSlice>>::new();
        let mut pages_indexes = Vec::<u32>::new();

        for cs in &self.cached_slices {
            if let CachedPageSlice::Cached(ref slice) = cs {
                if slice.dirty {
                    let slice_id = slice.layout.0;
                    let page_idx = slice.layout.1;

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
                let page_bytes = if let Some(bytes) = self.cached_pages.read_page(page_idx) {
                    bytes
                } else {
                    // TODO: add a const for page-size
                    Vec::with_capacity(4096)
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

        for (page_idx, page) in pages {
            self.cached_pages.write_page(page_idx, &page);
        }

        self.clear();
    }

    fn patch_page(&self, page: &mut Vec<u8>, slice: PageSlice) {
        //
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn loading_an_empty_slice_into_the_cache() {
        //
    }

    #[test]
    fn read_an_empty_slice_then_overriding_it_and_commit() {}

    #[test]
    fn read_an_existing_slice_then_overriding_it_and_commit() {}

    #[test]
    fn write_slice_without_loading_it_first_and_commit() {}

    #[test]
    fn write_slice_then_load_it_then_write_another_slice_under_the_same_page_and_commit() {}

    #[test]
    fn write_two_slices_under_same_page_and_commit() {}
}
