use std::fmt;

use crate::{Account, TemplateAddr};

/// Struct representation of the parsed raw Spawn-Account.
#[derive(PartialEq)]
pub struct SpawnApp {
    /// Transaction format version
    pub version: u16,

    /// Holds all `SpawnApp` non-ctor_name related data.
    pub app: Account,

    /// ctor function name
    pub ctor_name: String,

    /// calldata
    pub calldata: Vec<u8>,
}

#[doc(hidden)]
impl SpawnApp {
    pub fn app(&self) -> &Account {
        &self.app
    }

    pub fn app_name(&self) -> &str {
        &self.app.name
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        &self.app.template_addr
    }

    pub fn ctor_name(&self) -> &str {
        &self.ctor_name
    }

    pub fn ctor_data(&self) -> &[u8] {
        &self.calldata
    }
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
