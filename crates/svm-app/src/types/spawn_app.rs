use std::fmt;

use crate::types::{App, WasmValue};

/// Struct representation of the parsed raw Spawn-App.
#[derive(PartialEq)]
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

impl fmt::Debug for SpawnApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.app.fmt(f)?;
        writeln!(f, "ctor_idx: {}", self.ctor_idx)?;
        writeln!(
            f,
            "ctor_buf: {:?}",
            self.ctor_buf.iter().take(4).collect::<Vec<_>>()
        )?;
        writeln!(f, "ctor_args: {:?}", self.ctor_args)
    }
}
