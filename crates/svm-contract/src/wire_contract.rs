use crate::types::{CodeHash, Revision, Tag};
use svm_common::Address;

pub struct WireContract {
    pub(crate) name: String,
    pub(crate) wasm: Vec<u8>,
    pub(crate) tag: Tag,
    pub(crate) authors: Vec<Address>,
}
