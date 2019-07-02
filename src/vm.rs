/// this file will contain the `svm` functions that will be auto-imported into each `wasm` smart contract program
use wasmer_runtime::{func, imports, Ctx, ImportObject};

/// temporary vm system calls. The real vm system calls will be:
/// * crypto hashes (for example: sha3)
/// * contract storage (`store_kv` / `get_kv`)
/// * gas metering

#[doc(hidden)]
pub fn syscall_1(_ctx: &mut Ctx) -> i32 {
    return 10;
}

#[doc(hidden)]
pub fn syscall_2(_ctx: &mut Ctx, _a: i32, _b: i32) -> i32 {
    return 10;
}

/// this function receives an `import_object` and adds it the `svm` system calls
#[allow(unused)]
fn import_system_calls(import_object: &mut ImportObject) {
    let system_calls = imports! {
        "svm" => {
            "syscall_1" => func!(syscall_1),
            "syscall_2" => func!(syscall_2),
        },
    };

    import_object.extend(system_calls);
}
