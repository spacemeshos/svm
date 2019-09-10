use crate::page;
use crate::page::{PageIndex, PageSliceLayout};
use crate::traits::PageCache;
use std::collections::HashMap;
use svm_common::State;

#[derive(Debug, Clone, PartialEq)]
struct PageSlice {
    dirty: bool,
    data: Vec<u8>,
    layout: PageSliceLayout,
}

#[derive(Debug, Clone, PartialEq)]
enum CachedPageSlice {
    // We didn't load the page-slice yet from the underlying kv
    NotCached,

    // We've loaded the page-slice from the underlying kv and it had data
    Cached(PageSlice),
}

/// `PageSliceCache` is a caching layer on top of the `PageCache`.
/// While `PageCache` deals with data involving only page units, `PageSliceCache` has fine-grained
/// control for various sized of data.
pub struct PageSliceCache<PC: PageCache> {
    // The `ith item` will say whether the `ith page slice` is dirty
    cached_slices: Vec<CachedPageSlice>,

    page_cache: PC,
}

impl<PC: PageCache> std::fmt::Debug for PageSliceCache<PC> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "[DEBUG] PageCacheSlice")?;
        writeln!(f, "#Allocated slices: {}", self.cached_slices.len())?;

        for (i, slice) in self.cached_slices.iter().enumerate() {
            match slice {
                CachedPageSlice::NotCached => {
                    // skip
                }
                CachedPageSlice::Cached(ps) => {
                    writeln!(f, "Slice {}: has data", i)?;
                    writeln!(f, "   dirty: {}", ps.dirty)?;
                }
            }
        }

        writeln!(f)
    }
}

impl<PC: PageCache> PageSliceCache<PC> {
    /// Initializes a new `PageSliceCache` instance.
    ///
    /// * `page_cache` - implements the `PageCache` trait. In charge of supplying the pages
    ///  upon requests (`read_page`) and for propagating new pages versions (`write_page`).
    ///  However, persistence only takes place by triggerring `commit`
    ///
    /// * `max_pages_slices` - the maximum number of page-slices the `PageSliceCache` instance could
    ///   use when doing read / write. A page slice index is within the range `0..(max_pages_slices - 1)` (inclusive)
    pub fn new(page_cache: PC, max_pages_slices: usize) -> Self {
        Self {
            page_cache,
            cached_slices: vec![CachedPageSlice::NotCached; max_pages_slices],
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

                let page: Option<Vec<u8>> = self.page_cache.read_page(layout.page_idx);

                if let Some(page) = page {
                    let slice = PageSlice {
                        layout: layout.clone(),
                        dirty: false,
                        data: page.clone(),
                    };

                    std::mem::replace(
                        &mut self.cached_slices[slice_index],
                        CachedPageSlice::Cached(slice),
                    );

                    let start = layout.offset as usize;
                    let end = (layout.offset + layout.len) as usize;
                    let slice_data = page[start..end].to_vec();

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
            CachedPageSlice::Cached(slice) => Some(slice.data.to_vec()),
        }
    }

    /// * We insert the new page slice into `cached_slices` as `Cached(PageSlice)` and mark it as dirty
    pub fn write_page_slice(&mut self, layout: &PageSliceLayout, data: &[u8]) {
        // We don't mind whether the underlying page is already in the cache or not.
        // We just save the new written page-slice and mark it as `dirty`.

        let slice_index = layout.slice_idx.0 as usize;

        assert!(slice_index < self.cached_slices.len());

        let slice = PageSlice {
            layout: layout.clone(),
            dirty: true,
            data: data[0..(layout.len as usize)].to_vec(), // TODO: add a test checking for `layout.len`
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
        let max_pages_slices = self.cached_slices.len();

        self.cached_slices = vec![CachedPageSlice::NotCached; max_pages_slices];
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
    ///   after terimnating execution of the smart contract. The `clear` method is intended to be
    ///   used *only* for `tests`
    #[must_use]
    pub fn commit(&mut self) -> State {
        let mut page_slices = HashMap::<u32, Vec<PageSlice>>::new();
        let mut pages_indexes = Vec::<PageIndex>::new();

        for cs in &self.cached_slices {
            if let CachedPageSlice::Cached(ref slice) = cs {
                if slice.dirty {
                    let page_idx = slice.layout.page_idx;

                    let entry: &mut Vec<_> = page_slices.entry(page_idx.0).or_insert_with(Vec::new);
                    entry.push(slice.clone());

                    pages_indexes.push(page_idx);
                }
            }
        }

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
            .collect::<HashMap<PageIndex, Vec<u8>>>();

        for (page_idx, slices) in page_slices {
            let page = pages.get_mut(&PageIndex(page_idx)).unwrap();

            for slice in slices {
                self.patch_page(page, slice);
            }
        }

        // propagating the new versioned pages to `page_cache`
        for (page_idx, page) in pages {
            self.page_cache.write_page(page_idx, &page);
        }

        self.page_cache.commit();

        self.page_cache.get_state()
    }

    /// Applies `slice` on top of a `page`
    fn patch_page(&self, page: &mut Vec<u8>, slice: PageSlice) {
        let start = slice.layout.offset as usize;
        let end = (slice.layout.offset + slice.layout.len) as usize;

        let dst_slice = &mut page[start..end];

        dst_slice.copy_from_slice(&slice.data);
    }
}

impl<PC> Drop for PageSliceCache<PC>
where
    PC: PageCache,
{
    fn drop(&mut self) {
        dbg!("dropping `PageSliceCache`...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::default_page_hash;
    use svm_kv::traits::KVStore;

    use super::page::SliceIndex;

    fn fill_page(page: &mut [u8], items: &[(usize, u8)]) {
        for (i, b) in items {
            page[*i] = *b;
        }
    }

    macro_rules! page_slice_cache_gen {
        ($cache_slice_ident: ident, $kv_ident: ident, $addr: expr, $state: expr, $max_pages: expr, $max_pages_slices: expr) => {
            use crate::default::DefaultPageCache;
            use crate::memory::MemMerklePages;
            use svm_kv::memory::MemKVStore;

            use std::cell::RefCell;
            use std::rc::Rc;

            let $kv_ident = Rc::new(RefCell::new(MemKVStore::new()));
            let kv_gen = || Rc::clone(&$kv_ident);

            let pages = mem_merkle_pages_gen!($addr, $state, kv_gen, $max_pages);
            let cache = DefaultPageCache::<MemMerklePages>::new(pages, $max_pages);

            let mut $cache_slice_ident = PageSliceCache::new(cache, $max_pages_slices);
        };
    }

    macro_rules! mem_merkle_pages_gen {
        ($addr: expr, $state: expr, $kv_gen: expr, $max_pages: expr) => {{
            use crate::memory::MemMerklePages;
            use svm_common::{Address, State};

            let addr = Address::from($addr as u32);
            let state = State::from($state as u32);

            MemMerklePages::new(addr, $kv_gen(), state, $max_pages)
        }};
    }

    macro_rules! reopen_pages_storage {
        ($kv_ident: ident, $addr: expr, $state: expr, $max_pages: expr) => {{
            use crate::memory::MemMerklePages;
            use svm_common::Address;

            use std::rc::Rc;

            let addr = Address::from($addr as u32);
            MemMerklePages::new(addr, Rc::clone(&$kv_ident), $state, $max_pages)
        }};
    }

    macro_rules! reopen_page_slice_cache {
        ($cache_slice_ident: ident, $kv_ident: ident, $addr: expr, $state: expr, $max_pages: expr, $max_pages_slices: expr) => {
            let pages = reopen_pages_storage!($kv_ident, $addr, $state, $max_pages);

            let cache = crate::default::DefaultPageCache::<MemMerklePages>::new(pages, $max_pages);

            let mut $cache_slice_ident = PageSliceCache::new(cache, $max_pages_slices);
        };
    }

    #[test]
    fn loading_an_empty_slice_into_the_cache() {
        page_slice_cache_gen!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10, 100);

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
        page_slice_cache_gen!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10, 100);

        let layout = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 3,
        };

        assert_eq!(None, cache.read_page_slice(&layout));

        cache.write_page_slice(&layout, &vec![10, 20, 30]);

        assert_eq!(Some(vec![10, 20, 30]), cache.read_page_slice(&layout));

        // page is not persisted though since we didn't `commit`
        let ph = default_page_hash!(0x11_22_33_44, 0, &[10, 20, 30]);
        assert_eq!(None, kv.borrow().get(&ph.0));
    }

    #[test]
    fn write_slice_without_loading_it_first_and_commit() {
        let addr = 0x11_22_33_44;
        page_slice_cache_gen!(cache, kv, addr, 0x00_00_00_00, 2, 100);

        let layout = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 3,
        };

        cache.write_page_slice(&layout, &[10, 20, 30]);
        let new_state = cache.commit();

        // asserting persisted data. when viewing in the context of `new_state`.
        reopen_page_slice_cache!(cache, kv, addr, new_state, 2, 100);

        assert_eq!(Some(vec![10, 20, 30]), cache.read_page_slice(&layout));

        let mut expected_page = page::zero_page();
        fill_page(&mut expected_page, &[(100, 10), (101, 20), (102, 30)]);

        let ph = default_page_hash!(addr, 1, &expected_page);
        let actual_page = kv.borrow().get(&ph.0).unwrap();

        assert_eq!(expected_page, actual_page);
    }

    #[test]
    fn read_an_existing_slice_then_overriding_it_and_commit() {
        let addr = 0x11_22_33_44;
        page_slice_cache_gen!(cache, kv, addr, 0x00_00_00_00, 2, 100);

        let layout = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 3,
        };

        cache.write_page_slice(&layout, &vec![10, 20, 30]);
        let _ = cache.commit();

        let mut expected_page = page::zero_page();
        fill_page(&mut expected_page, &[(100, 10), (101, 20), (102, 30)]);
        let ph1 = default_page_hash!(addr, 1, &expected_page);
        fill_page(&mut expected_page, &[(100, 40), (101, 50), (102, 60)]);
        let ph2 = default_page_hash!(addr, 1, &expected_page);

        let page = kv.borrow().get(&ph1.0).unwrap();
        assert_eq!(vec![10, 20, 30], &page[100..103]);
        &cache.write_page_slice(&layout, &vec![40, 50, 60]);

        // new page is on the page-cache, but not persisted yet
        assert_eq!(Some(vec![40, 50, 60]), cache.read_page_slice(&layout));

        let page = kv.borrow().get(&ph1.0).unwrap();
        assert_eq!(vec![10, 20, 30], &page[100..103]);

        assert_eq!(None, kv.borrow().get(&ph2.0));

        // now we also persist the new page version
        let _ = cache.commit();

        let page = kv.borrow().get(&ph2.0).unwrap();
        assert_eq!(vec![40, 50, 60], &page[100..103]);
    }

    #[test]
    fn write_slice_and_commit_then_load_it_override_it_and_commit() {
        let addr = 0x11_22_33_44;
        page_slice_cache_gen!(cache, kv, addr, 0x00_00_00_00, 2, 100);

        let layout = PageSliceLayout {
            slice_idx: SliceIndex(0),
            page_idx: PageIndex(1),
            offset: 100,
            len: 3,
        };

        let mut expected_page = page::zero_page();
        fill_page(&mut expected_page, &[(100, 10), (101, 20), (102, 30)]);
        let ph1 = default_page_hash!(addr, 1, &expected_page);
        fill_page(&mut expected_page, &[(100, 40), (101, 50), (102, 60)]);
        let ph2 = default_page_hash!(addr, 1, &expected_page);

        // 1) first page write
        cache.write_page_slice(&layout, &vec![10, 20, 30]);

        // 2) commit
        let _ = cache.commit();

        // 3) load persisted page (we do a `clear` first to make sure we load from the page cache)
        cache.clear();

        assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout).unwrap());

        // 4) page override
        cache.write_page_slice(&layout, &vec![40, 50, 60]);
        assert_eq!(vec![40, 50, 60], cache.read_page_slice(&layout).unwrap());

        // 5) commit again
        let page = kv.borrow().get(&ph1.0).unwrap();
        assert_eq!(vec![10, 20, 30], &page[100..103]);

        let _ = cache.commit();

        let page = kv.borrow().get(&ph2.0).unwrap();
        assert_eq!(vec![40, 50, 60], &page[100..103]);
    }

    #[test]
    fn write_two_slices_under_same_page_and_commit() {
        let addr = 0x11_22_33_44;
        page_slice_cache_gen!(cache, kv, addr, 0x00_00_00_00, 2, 100);

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

        let mut expected_page = page::zero_page();
        fill_page(
            &mut expected_page,
            &[(100, 10), (101, 20), (102, 30), (200, 40), (201, 50)],
        );

        let ph = default_page_hash!(addr, 1, &expected_page);

        cache.write_page_slice(&layout1, &vec![10, 20, 30]);
        cache.write_page_slice(&layout2, &vec![40, 50]);

        assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout1).unwrap());
        assert_eq!(vec![40, 50], cache.read_page_slice(&layout2).unwrap());

        // commiting two slices under the same page
        assert_eq!(None, kv.borrow().get(&ph.0));

        let new_state = cache.commit();

        // asserting persisted data. when viewing in the context of `new_state`.
        reopen_page_slice_cache!(cache, kv, addr, new_state, 2, 100);

        assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout1).unwrap());
        assert_eq!(vec![40, 50], cache.read_page_slice(&layout2).unwrap());

        // queryind the key-value store directly
        let page = kv.borrow().get(&ph.0).unwrap();
        assert_eq!(vec![10, 20, 30], &page[100..103]);
        assert_eq!(vec![40, 50], &page[200..202]);
    }
}
