use std::convert::{TryFrom, TryInto};
use std::ffi::c_void;

use wasmer_c_api::wasm_c_api::{
    externals::{
        wasm_env_finalizer_t, wasm_extern_as_func, wasm_extern_t, wasm_func_as_extern,
        wasm_func_callback_t, wasm_func_new_with_env, wasm_func_t,
    },
    trap::wasm_trap_t,
    types::{wasm_functype_t, wasm_valkind_enum},
    value::{wasm_val_t, wasm_val_vec_t},
};

use svm_runtime::{Context, Import};

use wasmer::{Exportable, Function, FunctionType, RuntimeError, Store, Type, Val};
use wasmer_vm::{Export, VMFunctionBody, VMFunctionEnvironment};

pub unsafe fn wasm_func_new(
    callback: wasm_func_callback_t,
    func_sig: &FunctionType,
    store: &Store,
) -> Export {
    // https://github.com/wasmerio/wasmer/blob/7847acaae1e7a0eade13b65def1f3feeac95efd7/lib/c-api/src/wasm_c_api/externals/function.rs#L34

    let num_rets = func_sig.results().len();

    let inner_callback = move |args: &[Val]| -> Result<Vec<Val>, RuntimeError> {
        let processed_args: wasm_val_vec_t = args
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<wasm_val_t>, _>>()
            .expect("Argument conversion failed")
            .into();

        let zero = Val::I64(0);
        let zero = wasm_val_t::try_from(zero).unwrap();

        let mut results: wasm_val_vec_t = vec![zero; num_rets].into();

        let trap = callback(&processed_args, &mut results);

        if !trap.is_null() {
            let trap: Box<wasm_trap_t> = Box::from_raw(trap);

            todo!("raise `RuntimeError`")
            // return Err(trap.inner);
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

    let function = Function::new(store, func_sig, inner_callback);
    function.to_export()
}
