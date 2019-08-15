use crate::default::{DefaultPageIndexHasher, DefaultPagesStorage};
use crate::memory::MemKVStore;

/// A `PagesStorage` implementation backed by `MemKVStore`
pub type MemPages = DefaultPagesStorage<DefaultPageIndexHasher, MemKVStore>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::page::PageIndex;
    use crate::traits::PagesStorage;
    use svm_common::Address;

    use std::cell::RefCell;
    use std::rc::Rc;

    macro_rules! mem_kv_setup {
        ($kv: ident) => {
            let $kv = Rc::new(RefCell::new(MemKVStore::new()));
        };
    }

    macro_rules! mem_pages_setup {
        ($addr: expr, $kv: ident, $storage: ident) => {
            let addr = Address::from($addr as u32);

            let kv_clone = Rc::clone(&$kv);
            let mut $storage = MemPages::new(addr, kv_clone);
        };
    }

    macro_rules! mem_pages_and_kv_setup {
        ($addr: expr, $kv: ident, $storage: ident) => {
            let addr = Address::from($addr as u32);

            mem_kv_setup!($kv);
            let kv_clone = Rc::clone(&$kv);

            let mut $storage = MemPages::new(addr, kv_clone);
        };
    }

    #[test]
    fn a_page_does_not_exit_by_default() {
        mem_pages_and_kv_setup!(0x11_22_33_44, kv, storage);

        assert_eq!(None, storage.read_page(PageIndex(0)));
    }

    #[test]
    fn writing_a_page_does_not_auto_commit_it_to_underlying_kv() {
        mem_kv_setup!(kv);

        // both `storage1` and `storage2` service the same contract address `addr`
        // and both share the the same underlying key-value store
        mem_pages_setup!(0x11_22_33_44, kv, storage1);
        mem_pages_setup!(0x11_22_33_44, kv, storage2);

        // writing `page 0` with data `[10, 20, 30]`
        // changes aren't commited directly to `kv`
        storage1.write_page(PageIndex(0), &vec![10, 20, 30]);
        assert_eq!(None, storage1.read_page(PageIndex(0)));
        assert_eq!(None, storage2.read_page(PageIndex(0)));

        // another assertion for the uncommitted changes
        assert_eq!(1, storage1.uncommitted_len());
        assert_eq!(0, storage2.uncommitted_len());

        // now, storage `storage1` commits pending changes to `kv`
        storage1.commit();

        // both `storage1` and `storage2` report the same persisted `page 0`
        assert_eq!(vec![10, 20, 30], storage1.read_page(PageIndex(0)).unwrap());
        assert_eq!(vec![10, 20, 30], storage2.read_page(PageIndex(0)).unwrap());

        // no more pending changes
        assert_eq!(0, storage1.uncommitted_len());
        assert_eq!(0, storage2.uncommitted_len());
    }

    #[test]
    fn writing_the_same_page_twice_before_committing() {
        mem_pages_and_kv_setup!(0x11_22_33_44, kv, storage);

        // first write
        storage.write_page(PageIndex(0), &vec![10, 20, 30]);
        // one pending change
        assert_eq!(1, storage.uncommitted_len());

        // second write (page-override)
        storage.write_page(PageIndex(0), &vec![40, 50, 60]);
        // still, one pending change
        assert_eq!(1, storage.uncommitted_len());

        // commit page
        storage.commit();

        assert_eq!(vec![40, 50, 60], storage.read_page(PageIndex(0)).unwrap());
        // no pending changes
        assert_eq!(0, storage.uncommitted_len());
    }

    #[test]
    fn committing_the_same_page_under_two_different_contract_addresses() {
        mem_kv_setup!(kv);

        // `storagee1` and `storage2` share the same underlying `kv store`
        mem_pages_setup!(0x11_22_33_44, kv, storage1);
        mem_pages_setup!(0x55_66_77_88, kv, storage2);

        storage1.write_page(PageIndex(0), &vec![10, 20, 30]);
        storage2.write_page(PageIndex(0), &vec![40, 50, 60]);

        // committing pending changes
        storage1.commit();
        storage2.commit();

        // no more pending changes
        assert_eq!(0, storage1.uncommitted_len());
        assert_eq!(0, storage2.uncommitted_len());

        // two pages `[10, 20, 30]` and `[40, 50, 60]` have been committed successfully
        assert_eq!(vec![10, 20, 30], storage1.read_page(PageIndex(0)).unwrap());
        assert_eq!(vec![40, 50, 60], storage2.read_page(PageIndex(0)).unwrap());
    }
}
