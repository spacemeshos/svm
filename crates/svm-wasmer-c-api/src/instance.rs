use svm_wasmer::ctx::SvmCtx;
use svm_wasmer::*;

use svm_storage::{default::DefaultPageCache, memory::MemPages};

use wasmer_runtime::ImportObject;
use wasmer_runtime_c_api::import::wasmer_import_t;

pub type MemPageCache<'pc, K = [u8; 32]> = DefaultPageCache<'pc, MemPages<K>>;

include_wasmer_svm_vmcalls!(MemPageCache);

// #[no_mangle]
// pub unsafe extern "C" fn wasmer_append_svm_imports(
//     imports: *mut wasmer_import_t,
//     imports_len: libc::c_int,
// ) {
//     let import_object: &mut ImportObject = &mut *(imports as *mut ImportObject);
// }

#[cfg(test)]
mod tests {}
