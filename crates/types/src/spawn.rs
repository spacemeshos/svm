use std::fmt;

use crate::{Account, TemplateAddr};

/// Represents a `Spawn Account` transaction.
#[derive(PartialEq)]
pub struct SpawnAccount {
    account: Account,
    ctor_name: String,
    ctor_calldata: Vec<u8>,
}

#[doc(hidden)]
impl SpawnAccount {
    pub fn new(account: Account, ctor_name: String, ctor_calldata: Vec<u8>) -> Self {
        Self {
            account,
            ctor_name,
            ctor_calldata,
        }
    }

    pub fn app(&self) -> &Account {
        &self.account
    }

    pub fn account_name(&self) -> &str {
        self.account.name()
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        self.account.template_addr()
    }

    pub fn ctor_name(&self) -> &str {
        &self.ctor_name
    }

    pub fn ctor_calldata(&self) -> &[u8] {
        &self.ctor_calldata
    }
}

impl fmt::Debug for SpawnAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.account.fmt(f)?;

        writeln!(f, "ctor_name: {}", self.ctor_name)?;
        writeln!(
            f,
            "calldata: {:?}",
            self.ctor_calldata.iter().take(4).collect::<Vec<_>>()
        )
    }
}
