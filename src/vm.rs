/// this file will contain the `svm` functions that will be auto-imported into each `wasm` smart contract program

#[macro_use]
use wasmer_runtime::{Ctx, ImportObject, imports, func};

/// temporary vm system calls. The real vm system calls will be:
/// * crypto hashes (for example: sha3)
/// * contract storage (`store_kv` / `get_kv`)
/// * gas metering

#[doc(hidden)]
pub fn syscall_1(ctx: &mut Ctx) -> i32 {
    return 10;
}

#[doc(hidden)]
pub fn syscall_2(ctx: &mut Ctx, a: i32, b: i32) -> i32 {
    return 10;
}

/// this function receives an `import_object` and adds it the `svm` system calls
fn import_system_calls(import_object: &mut ImportObject) {
    let system_calls = imports! {
        "svm" => {
            "syscall_1" => func!(syscall_1),
            "syscall_2" => func!(syscall_2),
        },
    };

    import_object.extend(system_calls);
}
