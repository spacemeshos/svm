use std::collections::HashMap;
use std::marker::PhantomData;

use svm_types::{AccountAddr, Address, TemplateAddr};

use crate::env::{self, traits};

use env::ExtAccount;
use traits::{AccountDeserializer, AccountSerializer, AccountStore};

/// In-memory `AccountStore` implementation.
///
/// Should be used for mainly testing purposes only.
pub struct MemAccountStore<S, D> {
    acc_bytes: HashMap<Address, Vec<u8>>,
    phantom: PhantomData<(S, D)>,
}

impl<S, D> MemAccountStore<S, D>
where
    S: AccountSerializer,
    D: AccountDeserializer,
{
    /// Initializes a new [`MemAccountStore`]
    pub fn new() -> Self {
        Self {
            acc_bytes: HashMap::new(),
            phantom: PhantomData,
        }
    }
}

impl<S, D> AccountStore for MemAccountStore<S, D>
where
    S: AccountSerializer,
    D: AccountDeserializer,
{
    fn store(&mut self, account: &ExtAccount, addr: &AccountAddr) {
        let bytes = S::serialize(account);
        self.acc_bytes.insert(addr.inner().clone(), bytes);
    }

    fn load(&self, addr: &AccountAddr) -> Option<ExtAccount> {
        let bytes = self.acc_bytes.get(addr.inner());
        bytes.and_then(|bytes| D::deserialize(&bytes[..]))
    }

    fn resolve_template_addr(&self, addr: &AccountAddr) -> Option<TemplateAddr> {
        let account = self.load(addr);
        account.map(|x| x.template_addr().clone())
    }
}