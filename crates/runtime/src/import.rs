use std::convert::TryFrom;
use std::ffi::c_void;
use std::sync::Arc;

use crate::Context;

use wasmer::{
    Export, Exportable, Function, FunctionType, RuntimeError, Store, Type as WasmerType, Val,
};

use svm_ffi::{svm_byte_array, svm_env_t, svm_func_callback_t};
use svm_types::{Type, WasmType, WasmValue};

static WASMER_ARGS_STR: Type = Type::Str("Wasmer Args");

/// Holds data about an import of a function
/// conforming to the FFI interface
#[derive(Debug, Clone)]
pub struct ExternImport {
    name: String,

    namespace: String,

    params: Vec<WasmType>,

    returns: Arc<Vec<WasmType>>,

    func: svm_func_callback_t,

    host_env: *const c_void,
}

impl ExternImport {
    /// Creates a new struct
    pub fn new(
        name: String,
        namespace: String,
        params: Vec<WasmType>,
        returns: Vec<WasmType>,
        func: svm_func_callback_t,
        host_env: *const c_void,
    ) -> Self {
        Self {
            name,
            namespace,
            params,
            returns: Arc::new(returns),
            func,
            host_env,
        }
    }

    /// Wraps as a `Wasmer Export`
    pub fn wasmer_export(&self, store: &Store, ctx: &mut Context) -> (Export, *mut svm_env_t) {
        unsafe {
            // The following code has been highly influenced by code here:
            // https://github.com/wasmerio/wasmer/blob/dd69438efdd629a7b5ae8de53a774f177b0da48a/lib/c-api/src/wasm_c_api/externals/function.rs#L89

            let returns_types = self.returns.clone();
            let func = self.func;

            #[derive(wasmer::WasmerEnv, Clone)]
            struct WrapperEnv {
                func_env: *mut svm_env_t,
            }

            // SVM is single-threaded.
            // `Send`, `Sync` and `Clone` are required by `wasmer::WasmerEnv`
            unsafe impl Send for WrapperEnv {}
            unsafe impl Sync for WrapperEnv {}

            let wrapper_callback =
                move |env: &WrapperEnv, args: &[Val]| -> Result<Vec<Val>, RuntimeError> {
                    let args: Vec<WasmValue> = wasmer_vals_to_wasm_vals(args)?;
                    let args: svm_byte_array = (WASMER_ARGS_STR, args).into();

                    let mut results = svm_ffi::alloc_wasm_values(returns_types.len());

                    let err = func(env.func_env, &args, &mut results);

                    // manually releasing `args` internals
                    args.destroy();

                    if !err.is_null() {
                        // manually releasing `results` internals
                        results.destroy();

                        let err_ty = svm_ffi::SVM_WASM_ERROR_TYPE_PTR;
                        let err: svm_byte_array = svm_ffi::from_raw(err_ty, err);
                        let err_msg = String::try_from(&err);

                        let err_msg: String = match err_msg {
                            Ok(msg) => msg,
                            Err(..) => format!(
                                "Host function failed but error message isn't a valid UTF-8 String"
                            ),
                        };

                        err.destroy();

                        return Err(RuntimeError::new(err_msg));
                    }

                    let vals = parse_results(&results, &returns_types);

                    // manually releasing `results` internals
                    results.destroy();

                    vals.map(|vals| wasm_vals_to_wasmer_vals(&vals))
                };

            let func_ty = self.wasmer_function_ty();

            // making the input `&mut Context` appear as `*const c_void`
            let inner_env = ctx as *mut Context as *const c_void;
            let host_env = self.host_env;

            // The import used `env` (using Wasmer terminology) will be a struct of `svm_env_t`
            // This `#[repr(C)]` struct will contain two pointers to two types of `env`:
            //
            // 1. SVM inner env - a pointer to the `Context`
            //    Once SVM has finished executing a transaction its memory will be deallocated.
            //
            // 2. Host env - a pointer given as input by the so-called `Host`
            //    The responsibility of release that memory is up to the caller (the `host`).
            let func_env = svm_env_t {
                inner_env,
                host_env,
            };

            // The heap-allocated `func_env` will be deallocated by later by `SVM` running runtime.
            // (See method `funcs_envs_destroy` under `src/runtime/default.rs`)
            let ty = Type::of::<svm_env_t>();
            let func_env = svm_ffi::into_raw(ty, func_env) as *mut svm_env_t;

            let func =
                Function::new_with_env(store, &func_ty, WrapperEnv { func_env }, wrapper_callback);
            let export = func.to_export();

            (export, func_env)
        }
    }

    /// Returns the import's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the import's namespace
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    fn wasmer_function_ty(&self) -> FunctionType {
        let params = to_wasmer_types(&self.params);
        let returns = to_wasmer_types(&self.returns);

        FunctionType::new(params, returns)
    }
}

fn to_wasmer_types(types: &[WasmType]) -> Vec<WasmerType> {
    types
        .iter()
        .map(|ty| match ty {
            WasmType::I32 => WasmerType::I32,
            WasmType::I64 => WasmerType::I64,
        })
        .collect()
}

#[inline]
fn parse_results(
    bytes: &svm_byte_array,
    types: &[WasmType],
) -> Result<Vec<WasmValue>, RuntimeError> {
    let results = Vec::<WasmValue>::try_from(bytes);

    if results.is_err() {
        return Err(RuntimeError::new("Invalid results"));
    }

    let results = results.unwrap();

    if results.len() != types.len() {
        return Err(RuntimeError::new(format!(
            "Wrong number of #returns (expected: {}, actual: {})",
            types.len(),
            results.len()
        )));
    }

    for (i, (val, ty)) in results.iter().zip(types.iter()).enumerate() {
        if val.ty() != *ty {
            return Err(RuntimeError::new(format!(
                "Wrong WASM type for return value #{} (expected: {:?}, actual: {:?})",
                i,
                *ty,
                val.ty(),
            )));
        }
    }

    Ok(results)
}

#[inline]
fn wasmer_vals_to_wasm_vals(wasmer_vals: &[Val]) -> Result<Vec<WasmValue>, RuntimeError> {
    let mut values = Vec::new();

    for val in wasmer_vals {
        let value = match val {
            Val::I32(v) => WasmValue::I32(*v as u32),
            Val::I64(v) => WasmValue::I64(*v as u64),
            _ => return Err(RuntimeError::new("Invalid argument type")),
        };

        values.push(value);
    }

    Ok(values)
}

#[inline]
fn wasm_vals_to_wasmer_vals(vals: &[WasmValue]) -> Vec<Val> {
    vals.iter()
        .map(|val| match val {
            WasmValue::I32(v) => Val::I32(*v as i32),
            WasmValue::I64(v) => Val::I64(*v as i64),
        })
        .collect()
}
