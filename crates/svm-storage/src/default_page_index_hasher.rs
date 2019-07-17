use crate::page::PageIndex;
use crate::traits::PageIndexHasher;

use std::marker::PhantomData;
use std::ops::Add;

use svm_common::{Address, DefaultKeyHasher, KeyHasher};

pub struct PageIndexHasherImpl<H> {
    marker: PhantomData<H>,
}

impl<H: KeyHasher<Hash = [u8; 32]>> PageIndexHasher for PageIndexHasherImpl<H> {
    fn hash(address: Address, page: PageIndex) -> H::Hash {
        // `page_addr` is being allocated `33` and not `32` bytes due to possible addition carry
        let page_addr: [u8; 33] = address.add(page.0);

        H::hash(&page_addr)
    }
}

/// A default implementation for `PageIndex` hashing.
pub type DefaultPageIndexHasher = PageIndexHasherImpl<DefaultKeyHasher>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_page_hasher_sanity() {
        let expected = DefaultKeyHasher::hash(&[
            0x14, 0x22, 0x33, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]);

        let addr = Address::from(0x44_33_22_11 as u32);
        let page = PageIndex(3);

        let actual = DefaultPageIndexHasher::hash(addr, page);

        assert_eq!(expected, actual);
    }
}
