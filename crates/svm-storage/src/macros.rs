/// A utility macro for computing page-index hashes using `DefaultPageIndexHasher`
#[macro_export]
macro_rules! default_page_idx_hash {
    ($addr: expr, $page_idx: expr) => {{
        use crate::default::DefaultPageIndexHasher;
        use crate::page::PageIndex;
        use crate::traits::PageIndexHasher;
        use svm_common::Address;

        let addr = Address::from($addr as u32);

        DefaultPageIndexHasher::hash(addr, PageIndex($page_idx))
    }};
}

/// A utility macro for computing page hashes using `DefaultPageIndexHasher`
#[macro_export]
macro_rules! default_page_hash {
    ($addr: expr, $page_idx: expr, $page_data: expr) => {{
        use crate::default::DefaultPageHasher;
        use crate::page::PageIndex;
        use crate::traits::PageHasher;
        use svm_common::Address;

        let addr = Address::from($addr as u32);

        DefaultPageHasher::hash(addr, PageIndex($page_idx), $page_data)
    }};
}
