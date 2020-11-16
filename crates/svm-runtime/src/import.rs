use std::convert::{TryFrom, TryInto};
use std::ffi::c_void;
use std::ptr;

use crate::Context;

use svm_types::WasmType;

use wasmer::{Export, Exportable, Function, FunctionType, RuntimeError, Store, Type, Val};

use wasmer_c_api::wasm_c_api::{
    externals::{
        wasm_env_finalizer_t, wasm_extern_as_func, wasm_extern_t, wasm_func_as_extern,
        wasm_func_callback_t, wasm_func_new_with_env, wasm_func_t,
    },
    trap::wasm_trap_t,
    types::{wasm_functype_t, wasm_valkind_enum},
    value::{wasm_val_t, wasm_val_vec_t},
};

pub enum Import {
    Host(HostImport),
    Extern(ExternImport),
}

impl Import {
    pub fn namespace(&self) -> &str {
        match self {
            Import::Host(import) => &import.namespace,
            Import::Extern(import) => &import.namespace,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Import::Host(import) => &import.name,
            Import::Extern(import) => &import.name,
        }
    }
}

type WasmerHostFunction = dyn Fn(&mut Context, &[Val]) -> Result<Vec<Val>, RuntimeError> + 'static;

pub struct HostImport {
    pub name: String,

    pub namespace: String,

    pub params: Vec<WasmType>,

    pub returns: Vec<WasmType>,

    pub func: Box<WasmerHostFunction>,
}

#[derive(Debug, Clone)]
pub struct ExternImport {
    pub name: String,

    pub namespace: String,

    pub params: Vec<WasmType>,

    pub returns: Vec<WasmType>,

    pub func_ptr: *const c_void,
}

impl Import {
    pub fn wasmer_export(&self, store: &Store, context: Context) -> Export {
        match self {
            Import::Host(import) => todo!("this is for testing purposes"),
            Import::Extern(import) => {
                unsafe {
                    // This code is almost a clone of the code here:
                    // https://github.com/wasmerio/wasmer/blob/7847acaae1e7a0eade13b65def1f3feeac95efd7/lib/c-api/src/wasm_c_api/externals/function.rs#L34

                    let func_ty = self.wasmer_function_ty();
                    let callback: wasm_func_callback_t = std::mem::transmute(import.func_ptr);

                    let num_rets = func_ty.results().len();

                    let inner_callback = move |args: &[Val]| -> Result<Vec<Val>, RuntimeError> {
                        let processed_args: wasm_val_vec_t = args
                            .into_iter()
                            .map(TryInto::try_into)
                            .collect::<Result<Vec<wasm_val_t>, _>>()
                            .expect("Argument conversion failed")
                            .into();

                        let zero = wasm_val_t::try_from(Val::I64(0)).unwrap();
                        let mut results: wasm_val_vec_t = vec![zero; num_rets].into();

                        let trap = callback(&processed_args, &mut results);

                        if !trap.is_null() {
                            let trap: Box<wasm_trap_t> = Box::from_raw(trap);

                            // TODO: we want access to `trap.inner`
                            let err = RuntimeError::new("unexpected error");
                            return Err(err);
                        }

                        let processed_results = results
                            .into_slice()
                            .expect("Failed to convert `results` into a slice")
                            .into_iter()
                            .map(TryInto::try_into)
                            .collect::<Result<Vec<Val>, _>>()
                            .expect("Result conversion failed");

                        Ok(processed_results)
                    };

                    let function = Function::new(store, &func_ty, inner_callback);
                    function.to_export()
                }
            }
        }
    }

    fn wasmer_function_ty(&self) -> FunctionType {
        let (params, returns) = match self {
            Import::Host(import) => (&import.params, &import.returns),
            Import::Extern(import) => (&import.params, &import.returns),
        };

        let params = to_wasmer_types(params);
        let returns = to_wasmer_types(returns);

        FunctionType::new(params, returns)
    }
}

fn to_wasmer_types(types: &[WasmType]) -> Vec<Type> {
    types
        .iter()
        .map(|ty| match ty {
            WasmType::I32 => Type::I32,
            WasmType::I64 => Type::I64,
            _ => panic!("Only i32 and i64 are supported."),
        })
        .collect()
}
