use crate::{
    page::{PageAddr, PageIndex},
    traits::PageAddrHasher,
};

use svm_common::{Address, DefaultKeyHasher, KeyHasher};

/// Default `PageAddrHasher` implementation.
pub struct DefaultPageAddrHasher;

impl PageAddrHasher for DefaultPageAddrHasher {
    fn hash(app_addr: &Address, _page: PageIndex) -> PageAddr {
        let mut key = app_addr.bytes().to_vec();
        key.extend_from_slice(&[]);

        let hash = DefaultKeyHasher::hash(&key);
        let addr = &hash[0..Address::len()];

        Address::from(addr).into()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn default_page_hasher_sanity() {
//         let expected = DefaultKeyHasher::hash(&[
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x44, 0x33, 0x22, 0x14,
//         ]);

//         let addr = Address::from(0x44_33_22_11 as u32);
//         let page = PageIndex(3);

//         let actual = DefaultPageIndexHasher::hash(addr, page);

//         assert_eq!(expected, actual);
//     }
// }
