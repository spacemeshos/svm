use std::marker::PhantomData;
use std::ops::Add;

use crate::page::PageIndex;
use crate::traits::PageIndexHasher;

use svm_common::{Address, DefaultKeyHasher, KeyHasher};

pub struct PageIndexHasherImpl<H> {
    _phantom: PhantomData<H>,
}

impl<H: KeyHasher<Hash = [u8; 32]>> PageIndexHasher for PageIndexHasherImpl<H> {
    fn hash(app_addr: Address, page: PageIndex) -> H::Hash {
        // `page_addr` is being allocated `21` bytes and not `20` bytes due to possible additional carry
        let page_addr: [u8; 21] = app_addr.add(page.0.into());

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
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x44, 0x33, 0x22, 0x14,
        ]);

        let addr = Address::from(0x44_33_22_11 as u32);
        let page = PageIndex(3);

        let actual = DefaultPageIndexHasher::hash(addr, page);

        assert_eq!(expected, actual);
    }
}
