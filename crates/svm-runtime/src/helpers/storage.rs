use std::ffi::c_void;

use crate::contract_settings::ContractSettings;
use crate::helpers;

use svm_common::{Address, State};
use svm_storage::ContractStorage;

/// Extracts the `wasmer` instance context `data` field (of type `*mut c_void`) into `&mut ContractStorage`.
#[inline(always)]
pub fn wasmer_data_contract_storage<'a>(data: *const c_void) -> &'a mut ContractStorage {
    let svm_ctx = helpers::cast_ptr_to_svm_ctx(data);
    &mut svm_ctx.storage
}
