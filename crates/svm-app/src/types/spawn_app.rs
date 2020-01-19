use crate::types::{App, BufferSlice, WasmValue};

#[derive(Debug, PartialEq)]
pub struct SpawnApp {
    pub app: App,

    pub ctor_buf: Vec<BufferSlice>,

    pub ctor_args: Vec<WasmValue>,
}
