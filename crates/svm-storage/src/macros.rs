/// A utility macro for computing hashes using `DefaultPageIndexHasher`
#[macro_export]
macro_rules! default_page_hash {
    ($addr: expr, $page_idx: expr) => {{
        use crate::default::DefaultPageIndexHasher;
        use crate::page::PageIndex;
        use crate::traits::PageIndexHasher;
        use svm_common::Address;

        let addr = Address::from($addr as u32);

        DefaultPageIndexHasher::hash(addr, PageIndex($page_idx))
    }};
}
