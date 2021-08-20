use std::fmt;

use crate::{Account, TemplateAddr};

/// Struct representation of the parsed raw `Spawn Account` transaction.
#[derive(PartialEq)]
pub struct SpawnAccount {
    /// Transaction format version
    pub version: u16,

    /// Holds all [`SpawnAccount`] non-`ctor/calldata` related data.
    pub account: Account,

    /// ctor function name
    pub ctor_name: String,

    /// calldata
    pub calldata: Vec<u8>,
}

#[doc(hidden)]
impl SpawnAccount {
    pub fn new<V>(
        version: u16,
        template: &TemplateAddr,
        name: &str,
        ctor_name: &str,
        calldata: V,
    ) -> Self
    where
        V: Into<Vec<u8>>,
    {
        SpawnAccount {
            version,
            account: Account::new(template.clone(), name.to_string()),
            ctor_name: ctor_name.to_string(),
            calldata: calldata.into(),
        }
    }

    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn account_name(&self) -> &str {
        &self.account.name
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        &self.account.template_addr
    }

    pub fn ctor_name(&self) -> &str {
        &self.ctor_name
    }

    pub fn ctor_data(&self) -> &[u8] {
        &self.calldata
    }
}

impl fmt::Debug for SpawnAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.account.fmt(f)?;

        writeln!(f, "ctor_name: {}", self.ctor_name)?;
        writeln!(
            f,
            "calldata: {:?}",
            self.calldata.iter().take(4).collect::<Vec<_>>()
        )
    }
}
