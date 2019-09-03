use crate::page::PageIndex;
use crate::traits::{PageCache, PagesStorage};

#[derive(Debug, Clone)]
enum CachedPage {
    // We didn't load the page yet from the underlying db
    NotCached,

    // We've loaded page from the underlying db, but no data was there
    CachedEmpty,

    // We've loaded the page from the underlying db and it had data
    Cached(Vec<u8>),
}

/// `DefaultPageCache` serves us a cache layer for reading contract storage page.
/// In addition, it tracks dirty pages (pages that have been changed during the execution of a
/// smart contract).
pub struct DefaultPageCache<'ps, PS: PagesStorage> {
    // The `ith item` will say whether the `ith page` is dirty
    dirty_pages: Vec<bool>,

    // The `ith item` will say whether the `ith page` is cached
    cached_pages: Vec<CachedPage>,

    // The underlying storage pages
    pages_storage: &'ps mut PS,
}

impl<'ps, PS: PagesStorage> PageCache for DefaultPageCache<'ps, PS> {}

/// A `DefaultPageCache` is caching layer on top of a storage pages.
/// Each page change marks the page as dirty but the changes
/// are persisted to storage pages only upon `commit`
impl<'ps, PS: PagesStorage> DefaultPageCache<'ps, PS> {
    /// Initializes a new `DefaultPageCache` instance.
    ///
    /// * `pages_storage` - the underlying page-oriented page interface wrapping an underlying database.
    ///   doing a `pages_storage.commit()` should persist data to the underlying database.
    ///
    /// * `max_pages` - the maximum pages the `DefaultPageCache` instance could use when doing read / write.
    ///   A page index is within the range `0..(max_pages - 1)` (inclusive)
    pub fn new(pages_storage: &'ps mut PS, max_pages: usize) -> Self {
        Self {
            dirty_pages: vec![false; max_pages],
            cached_pages: vec![CachedPage::NotCached; max_pages],
            pages_storage,
        }
    }

    #[cfg(test)]
    fn is_dirty(&self, page_idx: usize) -> bool {
        self.dirty_pages[page_idx]
    }
}

impl<'ps, PS: PagesStorage> PagesStorage for DefaultPageCache<'ps, PS> {
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        // we can have an `assert` here since we are given the maximum storage-pages upon initialization
        assert!(self.cached_pages.len() > page_idx.0 as usize);

        let cache_status = &self.cached_pages[page_idx.0 as usize];

        match cache_status {
            CachedPage::NotCached => {
                // page isn't in the cache, so we delegate to `pages_storage`

                let page = self.pages_storage.read_page(page_idx);

                if page.is_some() {
                    let page: Vec<u8> = page.unwrap();

                    // we cache the loaded page
                    std::mem::replace(
                        &mut self.cached_pages[page_idx.0 as usize],
                        CachedPage::Cached(page.clone()),
                    );

                    Some(page)
                } else {
                    // page has no content under `pages_storage`
                    // we mark for the future it as `CachedEmpty`
                    std::mem::replace(
                        &mut self.cached_pages[page_idx.0 as usize],
                        CachedPage::CachedEmpty,
                    );

                    None
                }
            }
            CachedPage::Cached(page) => {
                // page has already been loaded from `pages_storage`.
                // since we it hascontent we clone `page` and return the clone
                Some(page.to_vec())
            }
            CachedPage::CachedEmpty => {
                // page has already been loaded from `pages_storage`.
                // but since we know it has no content we have nothing to do
                None
            }
        }
    }

    /// * we insert the new page content under `cached_pages`
    ///
    /// * we mark the page as dirty
    ///
    /// * we **don't** notify the underlying `pages_storage` about the page update.
    ///   only upon `commit`, we'll will propagate the `dirty pages` into `pages_storage`
    ///   we can do that since each future `read_page` will have a cache hit so we don't need to
    ///   ask `pages_storage` for data of an already cached page.
    fn write_page(&mut self, page_idx: PageIndex, page: &[u8]) {
        std::mem::replace(
            &mut self.cached_pages[page_idx.0 as usize],
            CachedPage::Cached(page.to_vec()),
        );

        std::mem::replace(&mut self.dirty_pages[page_idx.0 as usize], true);
    }

    /// * we clear both `dirty_pages` and `cached_pages`
    ///
    /// * we call `clear` on `pages_storage`
    ///
    /// Should be used for tests
    fn clear(&mut self) {
        for dirty in &mut self.dirty_pages {
            *dirty = false;
        }

        for page in &mut self.cached_pages {
            *page = CachedPage::NotCached;
        }

        self.pages_storage.clear();
    }

    /// * we traverse each page of `dirty_pages`, if it's not dirty we skip to the next page
    ///   if the page is `dirty` we take the corresponding page (it must be in the cache)
    ///   and we call `pages_storage.write_page` on that page
    ///
    /// * we call `pages_storage.commit` to flush the persist the changes
    ///
    /// since a smart contract is a short-lived program, we don't clear after `commit`
    fn commit(&mut self) {
        for ((page_idx, dirty), cached_page) in
            (&mut self.dirty_pages.iter().enumerate()).zip(&mut self.cached_pages.iter())
        {
            match *dirty {
                true => {
                    match cached_page {
                        CachedPage::Cached(ref page) => {
                            self.pages_storage
                                .write_page(PageIndex(page_idx as u32), page);
                        }
                        CachedPage::CachedEmpty | CachedPage::NotCached => {
                            // we should never reach this code!
                            //
                            // if a page is dirty then it's must appear in the cache.
                            // also we can't make a page dirty and `NotCached`
                            unreachable!()
                        }
                    }
                }
                false => {
                    // page isn't dirty, we skip for the next `cached_page`
                }
            }
        }

        self.pages_storage.commit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::MemKVStore;
    use crate::traits::KVStore;

    use crate::default_page_idx_hash;

    macro_rules! merkle_page_cache_gen {
        ($cache_ident: ident, $kv_ident: ident, $addr: expr, $state: expr, $max_pages: expr) => {
            use crate::default::DefaultPageCache;
            use crate::memory::{MemKVStore, MemMerklePages};

            use std::cell::RefCell;
            use std::sync::Arc;

            let $kv_ident = Arc::new(RefCell::new(MemKVStore::new()));
            let kv_gen = || Arc::clone(&$kv_ident);

            let mut pages = mem_merkle_pages_gen!($addr, $state, kv_gen, $max_pages);
            let mut $cache_ident = DefaultPageCache::<MemMerklePages>::new(&mut pages, $max_pages);
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

    #[test]
    fn loading_an_empty_page_into_the_cache() {
        merkle_page_cache_gen!(cache, db, 0x11_22_33_44, 0x00_00_00_00, 10);

        assert_eq!(None, cache.read_page(PageIndex(0)));
    }

    #[test]
    fn write_page_and_then_commit() {
        merkle_page_cache_gen!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10);

        let page = vec![10, 20, 30];

        cache.write_page(PageIndex(0), &page);
        assert_eq!(vec![10, 20, 30], cache.read_page(PageIndex(0)).unwrap());

        let ph = default_page_idx_hash!(0x11_22_33_44, 0);
        assert_eq!(None, kv.borrow().get(&ph));
    }

    #[test]
    #[ignore]
    fn writing_a_page_marks_it_as_dirty() {
        merkle_page_cache_gen!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10);

        assert_eq!(false, cache.is_dirty(0));

        let page = vec![10, 20, 30];
        cache.write_page(PageIndex(0), &page);

        assert_eq!(true, cache.is_dirty(0));
    }

    #[test]
    #[ignore]
    fn commit_persists_each_dirty_page() {
        merkle_page_cache_gen!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10);

        let page = vec![10, 20, 30];

        cache.write_page(PageIndex(0), &page);

        // `cache.write_page` doesn't persist the page yet
        let ph = default_page_idx_hash!(0x11_22_33_44, 0);
        assert_eq!(None, kv.borrow().get(&ph));

        cache.commit();

        // `cache.commit` persists the page
        assert_eq!(Some(vec![10, 20, 30]), kv.borrow().get(&ph));
    }
}
