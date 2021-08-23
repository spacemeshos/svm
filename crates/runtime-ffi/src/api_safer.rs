use log::{debug, error};

use std::ffi::c_void;

#[cfg(feature = "default-rocksdb")]
use std::path::Path;

use svm_codec::Codec;
use svm_runtime::Runtime;
use svm_types::{Context, Envelope, Type};

use crate::r#ref::RuntimeRef;

#[cfg(feature = "default-rocksdb")]
use crate::raw_utf8_error;

use crate::{raw_io_error, raw_validate_error};
use crate::{svm_byte_array, svm_resource_iter_t, svm_resource_t, svm_result_t, tracking};

pub static ENVELOPE_TYPE: Type = Type::Str("Tx Envelope");
pub static MESSAGE_TYPE: Type = Type::Str("Tx Message");
pub static CONTEXT_TYPE: Type = Type::Str("Tx Context");
pub static DEPLOY_RECEIPT_TYPE: Type = Type::Str("Deploy Receipt");
pub static SPAWN_RECEIPT_TYPE: Type = Type::Str("Spawn Receipt");
pub static VERIFY_RECEIPT_TYPE: Type = Type::Str("Verify Receipt");
pub static CALL_RECEIPT_TYPE: Type = Type::Str("Call Receipt");

pub static SVM_RESOURCE_TYPE: Type = Type::of::<svm_resource_t>();
pub static SVM_RESOURCES_ITER_TYPE: Type = Type::of::<svm_resource_iter_t>();
pub static SVM_RESOURCE_NAME_TYPE: Type = Type::Str("resource-name");
pub static SVM_RESOURCE_NAME_PTR_TYPE: Type = Type::Str("resource-name ptr");

pub fn svm_memory_runtime_create(runtime: &mut *mut c_void) -> svm_result_t {
    use svm_runtime::testing;

    debug!("`svm_memory_runtime_create` start");

    let mem_runtime = testing::create_memory_runtime();
    let res = into_raw_runtime(runtime, mem_runtime);

    debug!("`svm_memory_runtime_create` end");

    res
}

pub fn svm_envelope_alloc() -> svm_byte_array {
    let size = Envelope::fixed_size().unwrap();
    svm_byte_array::with_capacity(size, ENVELOPE_TYPE)
}

pub fn svm_message_alloc(size: u32) -> svm_byte_array {
    svm_byte_array::with_capacity(size as usize, MESSAGE_TYPE)
}

pub fn svm_context_alloc() -> svm_byte_array {
    let size = Context::fixed_size().unwrap();
    svm_byte_array::with_capacity(size, CONTEXT_TYPE)
}

pub fn svm_validate_deploy(
    runtime: &Box<dyn Runtime>,
    message: svm_byte_array,
    error: &mut svm_byte_array,
) -> svm_result_t {
    let message = message.as_slice();
    match runtime.validate_deploy(message) {
        Ok(()) => {
            debug!("`svm_validate_deploy` returns `SVM_SUCCESS`");
            svm_result_t::SVM_SUCCESS
        }
        Err(e) => {
            error!("`svm_validate_deploy` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

pub fn svm_validate_spawn(
    runtime: &Box<dyn Runtime>,
    message: svm_byte_array,
    error: &mut svm_byte_array,
) -> svm_result_t {
    let message = message.as_slice();

    match runtime.validate_spawn(message) {
        Ok(()) => {
            debug!("`svm_validate_spawn` returns `SVM_SUCCESS`");
            svm_result_t::SVM_SUCCESS
        }
        Err(e) => {
            error!("`svm_validate_spawn` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

pub fn svm_validate_call(
    runtime: &Box<dyn Runtime>,
    message: svm_byte_array,
    error: &mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_validate_call` start");

    let message = message.as_slice();

    match runtime.validate_call(message) {
        Ok(()) => {
            debug!("`svm_validate_call` returns `SVM_SUCCESS`");
            svm_result_t::SVM_SUCCESS
        }
        Err(e) => {
            error!("`svm_validate_call` returns `SVM_FAILURE`");
            raw_validate_error(&e, &mut *error);
            svm_result_t::SVM_FAILURE
        }
    }
}

pub fn svm_deploy(
    receipt: &mut svm_byte_array,
    runtime: &mut Box<dyn Runtime>,
    envelope: svm_byte_array,
    message: svm_byte_array,
    context: svm_byte_array,
    error: &mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_deploy` start`");

    let message = message.as_slice();

    let envelope = Envelope::decode_bytes(envelope);
    if let Err(e) = envelope {
        raw_io_error(e, &mut *error);
        return svm_result_t::SVM_FAILURE;
    }

    let context = Context::decode_bytes(context);
    if let Err(e) = context {
        raw_io_error(e, &mut *error);
        return svm_result_t::SVM_FAILURE;
    }

    let envelope = envelope.unwrap();
    let context = context.unwrap();
    let rust_receipt = runtime.deploy(&envelope, &message, &context);
    let receipt_bytes = rust_receipt.encode_to_vec();

    // returning encoded `TemplateReceipt` as `svm_byte_array`.
    //
    // # Notes
    //
    // Should call later `svm_receipt_destroy`
    data_to_svm_byte_array(DEPLOY_RECEIPT_TYPE, &mut *receipt, receipt_bytes);

    debug!("`svm_deploy` returns `SVM_SUCCESS`");
    svm_result_t::SVM_SUCCESS
}

pub fn svm_spawn(
    receipt: &mut svm_byte_array,
    runtime: &mut Box<dyn Runtime>,
    envelope: svm_byte_array,
    message: svm_byte_array,
    context: svm_byte_array,
    error: &mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_spawn` start");

    let message = message.as_slice();

    let envelope = Envelope::decode_bytes(envelope);
    if let Err(e) = envelope {
        raw_io_error(e, &mut *error);
        return svm_result_t::SVM_FAILURE;
    }

    let context = Context::decode_bytes(context);
    if let Err(e) = context {
        raw_io_error(e, &mut *error);
        return svm_result_t::SVM_FAILURE;
    }

    let envelope = envelope.unwrap();
    let context = context.unwrap();
    let rust_receipt = runtime.spawn(&envelope, &message, &context);
    let receipt_bytes = rust_receipt.encode_to_vec();

    // Returns the encoded `SpawnReceipt` as `svm_byte_array`.
    //
    // # Notes:
    //
    // Should call later `svm_receipt_destroy`
    data_to_svm_byte_array(SPAWN_RECEIPT_TYPE, &mut *receipt, receipt_bytes);

    debug!("`svm_spawn` returns `SVM_SUCCESS`");

    svm_result_t::SVM_SUCCESS
}

pub extern "C" fn svm_verify(
    receipt: &mut svm_byte_array,
    runtime: &mut Box<dyn Runtime>,
    envelope: svm_byte_array,
    message: svm_byte_array,
    context: svm_byte_array,
    error: &mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_verify` start");

    let message = message.as_slice();

    let envelope = Envelope::decode_bytes(envelope);
    if let Err(e) = envelope {
        raw_io_error(e, &mut *error);
        return svm_result_t::SVM_FAILURE;
    }

    let context = Context::decode_bytes(context);
    if let Err(e) = context {
        raw_io_error(e, &mut *error);
        return svm_result_t::SVM_FAILURE;
    }

    let envelope = envelope.unwrap();
    let context = context.unwrap();
    let rust_receipt = runtime.verify(&envelope, &message, &context);
    let receipt_bytes = rust_receipt.encode_to_vec();

    // Returns encoded `CallReceipt` as `svm_byte_array`.
    //
    // # Notes:
    //
    // Should call later `svm_receipt_destroy`
    data_to_svm_byte_array(VERIFY_RECEIPT_TYPE, &mut *receipt, receipt_bytes);

    debug!("`svm_verify` returns `SVM_SUCCESS`");
    svm_result_t::SVM_SUCCESS
}

pub fn svm_call(
    receipt: &mut svm_byte_array,
    runtime: &mut Box<dyn Runtime>,
    envelope: svm_byte_array,
    message: svm_byte_array,
    context: svm_byte_array,
    error: &mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_call` start");

    let message = message.as_slice();

    let envelope = Envelope::decode_bytes(envelope);
    if let Err(e) = envelope {
        raw_io_error(e, &mut *error);
        return svm_result_t::SVM_FAILURE;
    }

    let context = Context::decode_bytes(context);
    if let Err(e) = context {
        raw_io_error(e, &mut *error);
        return svm_result_t::SVM_FAILURE;
    }

    let envelope = envelope.unwrap();
    let context = context.unwrap();
    let rust_receipt = runtime.call(&envelope, &message, &context);
    let receipt_bytes = rust_receipt.encode_to_vec();

    // Returns encoded `CallReceipt` as `svm_byte_array`.
    //
    // # Notes:
    //
    // Should call later `svm_receipt_destroy`
    data_to_svm_byte_array(CALL_RECEIPT_TYPE, &mut *receipt, receipt_bytes);

    debug!("`svm_call` returns `SVM_SUCCESS`");
    svm_result_t::SVM_SUCCESS
}

pub fn svm_total_live_resources() -> i32 {
    std::panic::catch_unwind(tracking::total_live).unwrap_or(-1)
}

pub fn svm_resource_iter_new() -> *mut c_void {
    let ty = SVM_RESOURCES_ITER_TYPE;
    let snapshot = tracking::take_snapshot();

    crate::into_raw(ty, snapshot)
}

pub fn svm_resource_iter_next(iter: &mut svm_resource_iter_t) -> *mut svm_resource_t {
    match iter.next() {
        None => std::ptr::null_mut(),
        Some(resource) => {
            let ty = SVM_RESOURCE_TYPE;
            let ptr = crate::into_raw(ty, resource);

            ptr as *mut svm_resource_t
        }
    }
}

pub fn svm_resource_type_name_resolve(ty: usize) -> *mut svm_byte_array {
    match tracking::interned_type_rev(ty) {
        Some(ty) => {
            let ty = format!("{}", ty);
            let ty: svm_byte_array = (SVM_RESOURCE_NAME_TYPE, ty).into();

            let ptr = crate::into_raw(SVM_RESOURCE_NAME_PTR_TYPE, ty);
            ptr as _
        }
        None => std::ptr::null_mut(),
    }
}

#[inline]
fn data_to_svm_byte_array(ty: Type, byte_array: &mut svm_byte_array, data: Vec<u8>) {
    *byte_array = svm_byte_array::from((ty, data));
}

fn into_raw_runtime<R: Runtime + 'static>(
    raw_runtime: &mut *mut c_void,
    runtime: R,
) -> svm_result_t {
    let runtime_ptr = RuntimeRef::new(Box::new(runtime));

    // # Notes
    //
    // `svm_runtime_destroy` should be called later for freeing memory.
    *raw_runtime = RuntimeRef::into_raw(runtime_ptr);

    svm_result_t::SVM_SUCCESS
}
