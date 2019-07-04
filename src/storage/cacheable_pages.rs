use super::traits::{KVStore, PageHasher, PagesStorage};
use crate::Address;

type PageKey = [u8; 32];

#[derive(Debug, Clone)]
enum CachedPage {
    NotCached,

    CachedEmpty,

    Cached(Vec<u8>),
}

pub struct CacheablePages<'sp, PS: PagesStorage> {
    /// The `ith item` will say whether the `ith page` is dirty
    dirty_pages: Vec<bool>,

    /// The `ith item` will say whether the `ith page` is cached
    cached_pages: Vec<CachedPage>,

    /// The underlying storage pages
    storage_pages: &'sp mut PS,
}

impl<'sp, PS: PagesStorage> CacheablePages<'sp, PS> {
    fn new(storage_pages: &'sp mut PS, max_pages: usize) -> Self {
        Self {
            dirty_pages: vec![false; max_pages],

            cached_pages: vec![CachedPage::NotCached; max_pages],

            storage_pages,
        }
    }

    #[cfg(test)]
    fn is_dirty(&self, page_idx: usize) -> bool {
        self.dirty_pages[page_idx]
    }
}

impl<'sp, PS: PagesStorage> PagesStorage for CacheablePages<'sp, PS> {
    fn read_page(&mut self, page_idx: u32) -> Option<Vec<u8>> {
        // we can have an `assert` here since we are given the maximum storage-pages upon initialization
        assert!(self.cached_pages.len() > page_idx as usize);

        let cache_status = &self.cached_pages[page_idx as usize];

        match cache_status {
            CachedPage::NotCached => {
                // page isn't in the cache, so we delegate to `storage_pages`

                let page = self.storage_pages.read_page(page_idx);

                if page.is_some() {
                    let page: Vec<u8> = page.unwrap();

                    // we cache the loaded page
                    std::mem::replace(
                        &mut self.cached_pages[page_idx as usize],
                        CachedPage::Cached(page.clone()),
                    );

                    Some(page)
                } else {
                    // page has no content under `storage_pages`
                    // we mark for the future it as `CachedEmpty`
                    std::mem::replace(
                        &mut self.cached_pages[page_idx as usize],
                        CachedPage::CachedEmpty,
                    );

                    None
                }
            }
            CachedPage::Cached(page) => {
                // page has already been loaded from `storage_pages`.
                // since we it hascontent we clone `page` and return the clone
                Some(page.to_vec())
            }
            CachedPage::CachedEmpty => {
                // page has already been loaded from `storage_pages`.
                // but since we know it has no content we have nothing to do
                None
            }
        }
    }

    /// * we insert the new page content under `cached_pages`
    ///
    /// * we mark the page as dirty
    ///
    /// * we **don't** notify the underlying `storage_pages` about the page update.
    ///   only upon `commit`, we'll will propagate the `dirty pages` into `storage_pages`
    ///   we can do that since each future `read_page` will have a cache hit so we don't need to
    ///   ask `storage_pages` for data of an already cached page.
    fn write_page(&mut self, page_idx: u32, page: &[u8]) {
        std::mem::replace(
            &mut self.cached_pages[page_idx as usize],
            CachedPage::Cached(page.to_vec()),
        );

        std::mem::replace(&mut self.dirty_pages[page_idx as usize], true);
    }

    /// * we clear both `dirty_pages` and `cached_pages`
    /// * we call `clear` on `storage_pages`
    fn clear(&mut self) {
        for dirty in &mut self.dirty_pages {
            *dirty = false;
        }

        for page in &mut self.cached_pages {
            *page = CachedPage::NotCached;
        }

        self.storage_pages.clear();
    }

    /// * we traverse each page of `dirty_pages`, if it's not dirty we skip to the next page
    ///   if the page is `dirty` we take the corresponding page (it must be in the cache)
    ///   and we call `storage_pages.write_page` on that page
    ///
    /// * we call `storage_pages.commit` to flush the persist the changes
    ///
    /// * we call `clear`
    fn commit(&mut self) {
        for ((page_idx, dirty), cached_page) in
            (&mut self.dirty_pages.iter().enumerate()).zip(&mut self.cached_pages.iter())
        {
            match *dirty {
                true => {
                    match cached_page {
                        CachedPage::Cached(ref page) => {
                            self.storage_pages.write_page(page_idx as u32, page);
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

        self.storage_pages.commit();

        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::storage::traits::KVStore;
    use crate::storage::{DefaultPageHasher, MemKVStore, MemPagesStorage, PagesStorageImpl};
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type MemCacheablePages<'sp, K = [u8; 32]> = CacheablePages<'sp, MemPagesStorage<K>>;

    macro_rules! setup_cache {
        ($cache: ident, $db: ident, $addr: expr, $max_pages: expr) => {
            let addr = Address::from($addr as u32);

            let mut $db = Rc::new(RefCell::new(MemKVStore::new()));
            let mut db_clone = Rc::clone(&$db);

            let mut inner = MemPagesStorage::new(addr, db_clone);

            let mut $cache = MemCacheablePages::new(&mut inner, $max_pages);
        };
    }

    macro_rules! page_hash {
        ($addr: expr, $page_idx: expr) => {{
            let addr = Address::from($addr as u32);

            DefaultPageHasher::hash(addr, $page_idx)
        }};
    }

    #[test]
    fn loading_a_non_empty_page_into_the_cache() {
        setup_cache!(cache, db, 0x11_22_33_44, 10);

        assert_eq!(None, cache.read_page(0));
    }

    #[test]
    fn loading_a_empty_page_into_the_cache_and_then_doing_commit() {
        setup_cache!(cache, db, 0x11_22_33_44, 10);
        let page = vec![10, 20, 30];

        cache.write_page(0, &page);
        assert_eq!(vec![10, 20, 30], cache.read_page(0).unwrap());

        let ph = page_hash!(0x11_22_33_44, 0);
        assert_eq!(None, db.borrow().get(ph));
    }

    #[test]
    fn writing_a_page_marks_it_as_dirty() {
        setup_cache!(cache, db, 0x11_22_33_44, 10);

        assert_eq!(false, cache.is_dirty(0));

        let page = vec![10, 20, 30];
        cache.write_page(0, &page);

        assert_eq!(true, cache.is_dirty(0));
    }

    #[test]
    fn commit_persists_each_dirty_page() {
        setup_cache!(cache, db, 0x11_22_33_44, 10);
        let page = vec![10, 20, 30];

        cache.write_page(0, &page);

        // `cache.write_page` doesn't persist the page yet
        let ph = page_hash!(0x11_22_33_44, 0);
        assert_eq!(None, db.borrow().get(ph));

        cache.commit();

        // `cache.commit` persists the page
        assert_eq!(Some(vec![10, 20, 30]), db.borrow().get(ph));
    }
}
