use crate::types::{App, BufferSlice, WasmValue};

pub struct SpawnApp {
    pub app: App,

    pub ctor_buf_slices: Vec<BufferSlice>,

    pub ctor_args: Vec<WasmValue>,
}
