use super::traits::StoragePages;
use super::{DefaultPageHasher, MemKVStore, StoragePagesImpl};

pub type MemStoragePages = StoragePagesImpl<DefaultPageHasher, MemKVStore>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Address;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn page_is_empty_by_default() {
        let addr = Address::from(0x11_22_33_44 as u32);

        let mut kv = Rc::new(RefCell::new(MemKVStore::new()));
        let mut storage = MemStoragePages::new(addr, kv);

        assert_eq!(Vec::<u8>::new(), storage.read_page(0));
    }

    #[test]
    fn first_time_write_an_empty_page() {
        let addr = Address::from(0x11_22_33_44 as u32);

        let mut kv = Rc::new(RefCell::new(MemKVStore::new()));
        let mut storage = MemStoragePages::new(addr, kv);

        storage.write_page(0, &vec![10, 20, 30]);

        assert_eq!(vec![10, 20, 30], storage.read_page(0));
    }

    #[test]
    fn writing_the_same_page_under_two_different_addresses() {
        let addr1 = Address::from(0x11_22_33_44 as u32);
        let addr2 = Address::from(0x55_66_77_88 as u32);

        // we share the same underlying `kv store` within `storage1` and `storage2`
        let mut kv = Rc::new(RefCell::new(MemKVStore::new()));
        let mut kv_clone = Rc::clone(&kv);

        let mut storage1 = MemStoragePages::new(addr1, kv);
        let mut storage2 = MemStoragePages::new(addr2, kv_clone);

        storage1.write_page(0, &vec![10, 20, 30]);
        storage2.write_page(0, &vec![40, 50, 60]);

        assert_eq!(vec![10, 20, 30], storage1.read_page(0));
        assert_eq!(vec![40, 50, 60], storage2.read_page(0));
    }
}
