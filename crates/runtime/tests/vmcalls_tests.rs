use wasmer::imports;
use wasmer::{Function, NativeFunc};

use svm_layout::{FixedLayout, Id};
use svm_runtime::{testing, vmcalls, FuncEnv};
use svm_types::{Address, ReceiptLog};

/// Creates a new `Wasmer Store`
pub fn wasmer_store() -> wasmer::Store {
    svm_runtime::new_store()
}

/// Instantiate a `Wasmer` instance
pub fn wasmer_instantiate(
    store: &Store,
    import_object: &ImportObject,
    wasm_file: WasmFile,
) -> Instance {
    let module = wasmer_compile(store, wasm_file);

    Instance::new(&module, import_object).unwrap()
}

/// Creates a new `Wasmer Memory` consisting of a single page
/// The memory is of type non-shared and can grow unbounded.
fn wasmer_memory(store: &wasmer::Store) -> wasmer::Memory {
    let min = wasmer::Pages(1);
    let max = None;
    let shared = false;
    let ty = wasmer::MemoryType::new(min, max, shared);

    wasmer::Memory::new(store, ty).expect("Memory allocation has failed.")
}

macro_rules! assert_vars32 {
    ($instance:expr, $( $var_id:expr => $expected:expr), *) => {{
        __assert_vars_impl!(u32, $instance, $( $var_id => $expected), *)
    }};
}

macro_rules! assert_vars64 {
    ($instance:expr, $( $var_id:expr => $expected:expr), *) => {{
        __assert_vars_impl!(u64, $instance, $( $var_id => $expected), *)
    }};
}

macro_rules! __assert_vars_impl {
    ($ty:ty, $instance:expr, $( $var_id:expr => $expected:expr), *) => {{
        let func: &NativeFunc<u32, $ty> = &$instance.exports.get_native_function("get").unwrap();

        $( assert_eq!(func.call($var_id).unwrap(), $expected); )*
    }};
}

macro_rules! assert_storage {
    ($env:expr, $($var_id:expr => $expected:expr), *) => {{
        let storage = &$env.borrow().storage;

        $(
            let actual = storage.read_var(Id($var_id));
            assert_eq!(actual, $expected);
         )*
    }};
}

macro_rules! var_add32 {
    ($instance:expr, $var_id:expr, $amount:expr) => {{
        __var_add_impl!(u32, $instance, $var_id, $amount)
    }};
}

macro_rules! var_add64 {
    ($instance:expr, $var_id:expr, $amount:expr) => {{
        __var_add_impl!(u64, $instance, $var_id, $amount)
    }};
}

macro_rules! __var_add_impl {
    ($ty:ty, $instance:expr, $var_id:expr, $amount:expr) => {{
        let func: NativeFunc<(u32, $ty), ()> =
            $instance.exports.get_native_function("add").unwrap();

        let res = func.call($var_id, $amount);

        assert!(res.is_ok());
    }};
}

macro_rules! func {
    ($store:ident, $env:ident, $f:expr) => {{
        Function::new_native_with_env(&$store, $env.clone(), $f)
    }};
}

#[test]
fn vmcalls_empty_wasm() {
    let wasm = r#"
        (module
          (func (export "run")))"#
        .into();

    let store = wasmer_store();
    let import_object = imports! {};

    wasmer_instantiate(&store, &import_object, wasm);
}

#[test]
fn vmcalls_get32_set32() {
    let template_addr = Address::repeat(0xAB);
    let account_addr = Address::repeat(0xCD);
    let layout: FixedLayout = vec![4, 2].into();

    let store = wasmer_store();
    let storage = testing::blank_storage(&account_addr, &layout);

    let env = FuncEnv::new(storage, &template_addr.into(), &account_addr.into());

    let import_object = imports! {
        "svm" => {
            "svm_get32" => func!(store, env, vmcalls::get32),
            "svm_set32" => func!(store, env, vmcalls::set32),
        }
    };

    let instance = wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/get32_set32.wast").into(),
    );

    assert_vars32!(instance, 0 => 0, 1 => 0);

    var_add32!(instance, 0, 5); // adding 5 to var #0
    var_add32!(instance, 1, 10); // adding 10 to var #1

    assert_vars32!(instance, 0 => 5, 1 => 10);

    assert_storage!(env, 0 => [5, 0, 0, 0], 1 => [10, 0]);
}

#[test]
fn vmcalls_get64_set64() {
    let template_addr = Address::repeat(0xAB);
    let account_addr = Address::repeat(0xCD);
    let layout: FixedLayout = vec![4, 2].into();

    let store = wasmer_store();
    let storage = testing::blank_storage(&account_addr, &layout);
    let env = FuncEnv::new(storage, &template_addr.into(), &account_addr.into());

    let import_object = imports! {
        "svm" => {
            "svm_get64" => func!(store, env, vmcalls::get64),
            "svm_set64" => func!(store, env, vmcalls::set64),
        },
    };

    let instance = wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/get64_set64.wast").into(),
    );

    assert_vars64!(instance, 0 => 0, 1 => 0);

    var_add64!(instance, 0, 5); // adding 5 to var #0
    var_add64!(instance, 1, 10); // adding 10 to var #1

    assert_vars64!(instance, 0 => 5, 1 => 10);

    assert_storage!(env, 0 => [5, 0, 0, 0], 1 => [10, 0]);
}

#[test]
fn vmcalls_load160() {
    let template_addr = Address::repeat(0xAB);
    let account_addr = Address::repeat(0xCD);
    let layout: FixedLayout = vec![20].into();

    let store = wasmer_store();
    let memory = wasmer_memory(&store);
    let storage = testing::blank_storage(&account_addr, &layout);
    let env = FuncEnv::new_with_memory(
        memory.clone(),
        storage,
        &template_addr.into(),
        &account_addr.clone().into(),
    );

    let import_object = imports! {
        "svm" => {
            "memory" => memory.clone(),
            "svm_load160" => func!(store, env, vmcalls::load160),
            "svm_store160" => func!(store, env, vmcalls::store160),
        },
    };

    let instance = wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/load160_store160.wast").into(),
    );

    {
        let storage = &mut env.borrow_mut().storage;
        storage.write_var(Id(0), account_addr.as_slice().to_vec());
    }

    let func: NativeFunc<(u32, u32)> = instance.exports.get_native_function("load").unwrap();
    let ptr = 0;
    let var_id = 0;

    func.call(var_id, ptr).expect("function has failed");

    let view = &memory.view::<u8>()[ptr as usize..(ptr as usize + 20)];
    let bytes: Vec<u8> = view.iter().map(|cell| cell.get()).collect();

    assert_eq!(account_addr, Address::from(&bytes[..]));
}

#[test]
fn vmcalls_store160() {
    let template_addr = Address::repeat(0xAB);
    let account_addr = Address::repeat(0xCD);
    let layout: FixedLayout = vec![20].into();

    let store = wasmer_store();
    let memory = wasmer_memory(&store);
    let storage = testing::blank_storage(&account_addr, &layout);
    let env = FuncEnv::new_with_memory(
        memory.clone(),
        storage,
        &template_addr.into(),
        &account_addr.clone().into(),
    );

    let import_object = imports! {
        "svm" => {
            "memory" => memory.clone(),
            "svm_load160" => func!(store, env, vmcalls::load160),
            "svm_store160" => func!(store, env, vmcalls::store160),
        },
    };

    let instance = wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/load160_store160.wast").into(),
    );

    for (cell, byte) in memory.view::<u8>().iter().zip(account_addr.as_slice()) {
        cell.set(*byte);
    }

    let func: NativeFunc<(u32, u32)> = instance.exports.get_native_function("store").unwrap();
    let ptr = 0;
    let var_id = 0;

    func.call(var_id, ptr).expect("function has failed");

    assert_storage!(env, 0 => account_addr.as_slice());
}

#[test]
fn vmcalls_log() {
    let template_addr = Address::repeat(0xAB);
    let account_addr = Address::repeat(0xCD);
    let layout = FixedLayout::default();

    let store = wasmer_store();
    let memory = wasmer_memory(&store);
    let storage = testing::blank_storage(&account_addr, &layout);
    let env = FuncEnv::new_with_memory(
        memory.clone(),
        storage,
        &template_addr.into(),
        &account_addr.into(),
    );

    let import_object = imports! {
        "svm" => {
            "memory" => memory.clone(),
            "svm_log" => func!(store, env, vmcalls::log),
        },
    };

    let instance = wasmer_instantiate(&store, &import_object, include_str!("wasm/log.wast").into());

    let data = b"Hello World";

    for (cell, byte) in memory.view::<u8>().iter().zip(data) {
        cell.set(*byte);
    }

    let logs = env.borrow_mut().take_logs();
    assert!(logs.is_empty());

    let func = instance.exports.get_function("sayHello").unwrap();
    let _ = func.call(&[]).unwrap();

    let logs = env.borrow_mut().take_logs();

    assert_eq!(
        logs,
        vec![ReceiptLog {
            msg: b"Hello World".to_vec(),
            code: 200
        }]
    );
}
