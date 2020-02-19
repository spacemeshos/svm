use crate::types::{App, WasmValue};

#[derive(Debug, PartialEq)]
pub struct SpawnApp {
    pub app: App,

    pub ctor_idx: u16,

    pub ctor_buf: Vec<u8>,

    pub ctor_args: Vec<WasmValue>,
}
