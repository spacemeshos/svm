use std::fmt;

use crate::App;

/// Struct representation of the parsed raw Spawn-App.
#[derive(PartialEq)]
pub struct SpawnApp {
    /// Transaction format version
    pub version: u16,

    /// Holds all `SpawnApp` non-ctor_name related data.
    pub app: App,

    /// ctor function name
    pub ctor_name: String,

    /// calldata
    pub calldata: Vec<u8>,
}

impl fmt::Debug for SpawnApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.app.fmt(f)?;

        writeln!(f, "ctor_name: {}", self.ctor_name)?;
        writeln!(
            f,
            "calldata: {:?}",
            self.calldata.iter().take(4).collect::<Vec<_>>()
        )
    }
}
