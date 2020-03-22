use crate::types::{App, WasmValue};

/// Struct representation of the parsed raw Spawn-App.
#[derive(Debug, PartialEq)]
pub struct SpawnApp {
    /// Holds all `SpawnApp` non-ctor related data.
    pub app: App,

    /// ctor function index
    pub ctor_idx: u16,

    /// ctor function buffer
    pub ctor_buf: Vec<u8>,

    /// ctor function args
    pub ctor_args: Vec<WasmValue>,
}
