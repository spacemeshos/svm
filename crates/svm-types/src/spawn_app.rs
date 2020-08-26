use std::fmt;

use crate::App;

/// Struct representation of the parsed raw Spawn-App.
#[derive(PartialEq)]
pub struct SpawnApp {
    /// Holds all `SpawnApp` non-ctor related data.
    pub app: App,

    /// ctor function name
    pub ctor: String,

    /// calldata
    pub calldata: Vec<u8>,
}

impl fmt::Debug for SpawnApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.app.fmt(f)?;

        writeln!(f, "ctor: {}", self.ctor)?;
        writeln!(
            f,
            "calldata: {:?}",
            self.calldata.iter().take(4).collect::<Vec<_>>()
        )
    }
}
