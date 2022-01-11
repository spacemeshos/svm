use std::ops::AddAssign;

use wasmer::{imports, FromToNativeWasmType, NativeFunc};

use svm_genesis_config::GenesisConfig;
use svm_layout::FixedLayout;
use svm_runtime::{vmcalls, AccessMode, FuncEnv};
use svm_runtime_testing::WasmFile;
use svm_state::{AccountStorage, GlobalState};
use svm_types::{Address, BytesPrimitive, Context, Envelope, ReceiptLog, TemplateAddr};

fn create_account(
    gs: GlobalState,
    addr: &Address,
    template_addr: &TemplateAddr,
    layout: FixedLayout,
) -> AccountStorage {
    use svm_layout::Layout;
    use svm_state::TemplateStorage;
    use svm_types::*;

    let code_section = CodeSection::new(
        svm_types::CodeKind::Wasm,
        vec![],
        0,
        svm_types::GasMode::Fixed,
        0,
    );
    let data_section = DataSection::with_layout(Layout::Fixed(layout));
    let ctors_section = CtorsSection::new(vec![]);

    let core_sections = Template::new(code_section, data_section, ctors_section)
        .sections()
        .clone();
    let noncore_sections = Sections::with_capacity(0);

    TemplateStorage::create(gs.clone(), &template_addr, core_sections, noncore_sections).unwrap();

    AccountStorage::create(gs, addr, "NAME".to_string(), template_addr.clone(), 0, 0).unwrap()
}

/// Creates a new `Wasmer Store`
pub fn wasmer_store() -> wasmer::Store {
    svm_runtime::new_store()
}

/// Compiles a Wasm program in textual format (a.k.a Wast) into a [`wasmer::Module`].
pub fn wasmer_compile(store: &wasmer::Store, wasm_file: WasmFile) -> wasmer::Module {
    let wasm = wasm_file.into_bytes();

    wasmer::Module::from_binary(&store, &wasm[..]).unwrap()
}

/// Instantiate a `Wasmer` instance
pub fn wasmer_instantiate(
    store: &wasmer::Store,
    import_object: &wasmer::ImportObject,
    wasm_file: WasmFile,
) -> wasmer::Instance {
    let module = wasmer_compile(store, wasm_file);

    wasmer::Instance::new(&module, import_object).unwrap()
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

fn assert_var_eq<T>(instance: &wasmer::Instance, var_id: u32, expected: T)
where
    T: FromToNativeWasmType + PartialEq + std::fmt::Debug,
{
    let func: &NativeFunc<u32, T> = &instance.exports.get_native_function("get").unwrap();

    assert_eq!(func.call(var_id).unwrap(), expected);
}

fn assert_storage(env: &FuncEnv, var_id: u32, expected: impl Into<Vec<u8>>) {
    let mut borrow = env.borrow_mut();
    let storage = borrow.storage_mut();

    assert_eq!(storage.get_var_vec(var_id).unwrap(), expected.into());
}

fn var_add<T>(instance: &wasmer::Instance, var_id: u32, amount: T)
where
    T: FromToNativeWasmType + AddAssign,
{
    let func: &NativeFunc<(u32, T), ()> = &instance.exports.get_native_function("add").unwrap();
    let res = func.call(var_id, amount);

    assert!(res.is_ok());
}

macro_rules! func {
    ($store:ident, $env:ident, $f:expr) => {{
        wasmer::Function::new_native_with_env(&$store, $env.clone(), $f)
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
    let template_addr = TemplateAddr::repeat(0xAB);
    let target_addr = Address::repeat(0xCD);
    let layout = FixedLayout::from_byte_sizes(0, &[4, 2]);

    let gs = GlobalState::in_memory(GenesisConfig::mainnet());
    let store = wasmer_store();
    let storage = create_account(gs, &target_addr, &template_addr, layout);
    let envelope = Envelope::default();
    let context = Context::default();
    let func_env = FuncEnv::new(
        storage,
        &envelope,
        &context,
        template_addr,
        target_addr,
        AccessMode::FullAccess,
    );

    let import_object = imports! {
        "svm" => {
            "svm_get32" => func!(store, func_env, vmcalls::get32),
            "svm_set32" => func!(store, func_env, vmcalls::set32),
        }
    };

    let instance = wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/get32_set32.wast").into(),
    );

    assert_var_eq(&instance, 0, 0u32);
    assert_var_eq(&instance, 1, 0u32);

    var_add(&instance, 0, 5u32);
    var_add(&instance, 1, 10u32);

    assert_var_eq(&instance, 0, 5u32);
    assert_var_eq(&instance, 1, 10u32);

    assert_storage(&func_env, 0, [5, 0, 0, 0]);
    assert_storage(&func_env, 1, [10, 0]);
}

#[test]
fn vmcalls_get64_set64() {
    let template_addr = TemplateAddr::repeat(0xAB);
    let target_addr = Address::repeat(0xCD);
    let layout = FixedLayout::from_byte_sizes(0, &[4, 2]);

    let gs = GlobalState::in_memory(GenesisConfig::mainnet());
    let store = wasmer_store();
    let storage = create_account(gs, &target_addr, &template_addr, layout);
    let envelope = Envelope::default();
    let context = Context::default();
    let func_env = FuncEnv::new(
        storage,
        &envelope,
        &context,
        template_addr,
        target_addr,
        AccessMode::FullAccess,
    );

    let import_object = imports! {
        "svm" => {
            "svm_get64" => func!(store, func_env, vmcalls::get64),
            "svm_set64" => func!(store, func_env, vmcalls::set64),
        },
    };

    let instance = wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/get64_set64.wast").into(),
    );

    assert_var_eq(&instance, 0, 0u64);
    assert_var_eq(&instance, 1, 0u64);

    var_add(&instance, 0, 5u64);
    var_add(&instance, 1, 10u64);

    assert_var_eq(&instance, 0, 5u64);
    assert_var_eq(&instance, 1, 10u64);

    assert_storage(&func_env, 0, [5, 0, 0, 0]);
    assert_storage(&func_env, 1, [10, 0]);
}

#[test]
fn vmcalls_load160() {
    let template_addr = TemplateAddr::repeat(0xAB);
    let target_addr = Address::repeat(0xCD);
    let layout = FixedLayout::from_byte_sizes(0, &[20]);

    let gs = GlobalState::in_memory(GenesisConfig::mainnet());
    let store = wasmer_store();
    let memory = wasmer_memory(&store);
    let storage = create_account(gs, &target_addr, &template_addr, layout);
    let envelope = Envelope::default();
    let context = Context::default();
    let func_env = FuncEnv::new_with_memory(
        memory.clone(),
        storage,
        &envelope,
        &context,
        template_addr,
        target_addr.clone(),
        AccessMode::FullAccess,
    );

    let import_object = imports! {
        "svm" => {
            "memory" => memory.clone(),
            "svm_load160" => func!(store, func_env, vmcalls::load160),
            "svm_store160" => func!(store, func_env, vmcalls::store160),
        },
    };

    let instance = wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/load160_store160.wast").into(),
    );

    {
        let mut borrow = func_env.borrow_mut();
        let storage = borrow.storage_mut();
        storage.set_var_bytes(0, target_addr.as_slice()).unwrap();
    }

    let func: NativeFunc<(u32, u32)> = instance.exports.get_native_function("load").unwrap();
    let ptr = 0;
    let var_id = 0;

    func.call(var_id, ptr).expect("function has failed");

    let view = &memory.view::<u8>()[ptr as usize..(ptr as usize + 20)];
    let bytes: Vec<u8> = view.iter().map(|cell| cell.get()).collect();

    assert_eq!(target_addr, Address::new(&bytes[..]));
}

#[test]
fn vmcalls_store160() {
    let template_addr = TemplateAddr::repeat(0xAB);
    let target_addr = Address::repeat(0xCD);
    let layout = FixedLayout::from_byte_sizes(0, &[20]);

    let gs = GlobalState::in_memory(GenesisConfig::mainnet());
    let store = wasmer_store();
    let memory = wasmer_memory(&store);
    let storage = create_account(gs, &target_addr, &template_addr, layout);
    let envelope = Envelope::default();
    let context = Context::default();
    let func_env = FuncEnv::new_with_memory(
        memory.clone(),
        storage,
        &envelope,
        &context,
        template_addr,
        target_addr.clone(),
        AccessMode::FullAccess,
    );

    let import_object = imports! {
        "svm" => {
            "memory" => memory.clone(),
            "svm_load160" => func!(store, func_env, vmcalls::load160),
            "svm_store160" => func!(store, func_env, vmcalls::store160),
        },
    };

    let instance = wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/load160_store160.wast").into(),
    );

    for (cell, byte) in memory.view::<u8>().iter().zip(target_addr.as_slice()) {
        cell.set(*byte);
    }

    let func: NativeFunc<(u32, u32)> = instance.exports.get_native_function("store").unwrap();
    let ptr = 0;
    let var_id = 0;

    func.call(var_id, ptr).expect("function has failed");

    assert_storage(&func_env, 0, target_addr.as_slice());
}

#[test]
fn vmcalls_log() {
    let template_addr = TemplateAddr::repeat(0xAB);
    let target_addr = Address::repeat(0xCD);
    let layout = FixedLayout::default();

    let gs = GlobalState::in_memory(GenesisConfig::mainnet());
    let store = wasmer_store();
    let memory = wasmer_memory(&store);
    let storage = create_account(gs, &target_addr, &template_addr, layout);
    let envelope = Envelope::default();
    let context = Context::default();
    let func_env = FuncEnv::new_with_memory(
        memory.clone(),
        storage,
        &envelope,
        &context,
        template_addr,
        target_addr,
        AccessMode::AccessDenied,
    );

    let import_object = imports! {
        "svm" => {
            "memory" => memory.clone(),
            "svm_log" => func!(store, func_env, vmcalls::log),
        },
    };

    let instance = wasmer_instantiate(&store, &import_object, include_str!("wasm/log.wast").into());

    let data = b"Hello World";

    for (cell, byte) in memory.view::<u8>().iter().zip(data) {
        cell.set(*byte);
    }

    let logs = func_env.borrow_mut().take_logs();
    assert!(logs.is_empty());

    let func = instance.exports.get_function("sayHello").unwrap();
    let _ = func.call(&[]).unwrap();

    let logs = func_env.borrow_mut().take_logs();
    assert_eq!(logs, vec![ReceiptLog::new(b"Hello World".to_vec(),)]);
}

fn setup_svm_transfer_test() -> (
    NativeFunc<(u32, u32, i64)>,
    u32,
    u32,
    AccountStorage,
    AccountStorage,
) {
    let src_addr = Address::repeat(0xCD);
    let dst_addr = Address::repeat(0x11);
    let template = TemplateAddr::stub();

    let layout = FixedLayout::from_byte_sizes(0, &[20]);
    let store = wasmer_store();
    let memory = wasmer_memory(&store);
    let gs = GlobalState::in_memory(GenesisConfig::mainnet());

    let mut src_account = create_account(gs.clone(), &src_addr, &template, layout.clone());
    src_account.set_balance(1000).unwrap();
    assert_eq!(src_account.balance().unwrap(), 1000);

    let dst_account = create_account(gs, &dst_addr, &template, layout);

    let envelope = Envelope::default();
    let context = Context::default();
    let src_func_env = FuncEnv::new_with_memory(
        memory.clone(),
        src_account.clone(),
        &envelope,
        &context,
        template,
        src_addr.clone(),
        AccessMode::FullAccess,
    );

    let import_object = imports! {
        "svm" => {
            "svm_transfer" => func!(store, src_func_env, vmcalls::svm_transfer),
        },
    };

    let instance = wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/svm_transfer.wast").into(),
    );

    let func: NativeFunc<(u32, u32, i64)> =
        instance.exports.get_native_function("transfer").unwrap();
    let src_addr_ptr = 0u32;
    let dst_addr_ptr = 20u32;

    for (cell, byte) in memory
        .view::<u8>()
        .iter()
        .skip(src_addr_ptr as usize)
        .zip(src_addr.as_slice())
    {
        cell.set(*byte);
    }

    for (cell, byte) in memory
        .view::<u8>()
        .iter()
        .skip(dst_addr_ptr as usize)
        .zip(dst_addr.as_slice())
    {
        cell.set(*byte);
    }

    (func, src_addr_ptr, dst_addr_ptr, src_account, dst_account)
}

#[test]
fn vmcalls_svm_transfer() {
    let (func, src_addr_ptr, dst_addr_ptr, src_account, dst_account) = setup_svm_transfer_test();

    func.call(src_addr_ptr, dst_addr_ptr, 100).unwrap();
    assert_eq!(src_account.balance().unwrap(), 900);
    assert_eq!(dst_account.balance().unwrap(), 100);

    func.call(src_addr_ptr, dst_addr_ptr, 900).unwrap();
    assert_eq!(src_account.balance().unwrap(), 0);
    assert_eq!(dst_account.balance().unwrap(), 1000);
}

#[test]
#[should_panic]
fn vmcalls_svm_transfer_insufficient_funds() {
    let (func, src_addr_ptr, dst_addr_ptr, _, _) = setup_svm_transfer_test();

    func.call(src_addr_ptr, dst_addr_ptr, 1001).ok();
}
