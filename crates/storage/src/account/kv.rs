use std::sync::{Arc, Mutex, MutexGuard};

use crate::kv::StatefulKV;

use svm_hash::{Blake3Hasher, Hasher};
use svm_types::{Address, BytesPrimitive, State};

/// An Account-aware (and `State`-aware) key-value store interface responsible of
/// mapping `u32` input keys (given as a 4 byte-length slice) to global keys under a raw key-value store.
///
/// The mapping is dependant on the contextual `Account`'s `Address` (see the `new` method).
pub struct AccountKVStore {
    pub(crate) account_addr: Address,

    pub(crate) kv: Arc<Mutex<dyn StatefulKV + Send>>,
}

impl StatefulKV for AccountKVStore {
    #[inline]
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let key = self.build_key(key);
        self.kv().get(&key)
    }

    #[inline]
    fn set(&mut self, key: &[u8], value: &[u8]) {
        let key = self.build_key(key);

        self.kv().set(&key, value);
    }

    #[inline]
    fn discard(&mut self) {
        self.kv().discard();
    }

    #[inline]
    fn flush(&mut self) {
        self.kv().flush();
    }

    #[inline]
    #[must_use]
    fn checkpoint(&mut self) -> State {
        self.kv().checkpoint()
    }

    #[inline]
    #[must_use]
    fn rewind(&mut self, state: &State) {
        self.kv().rewind(state)
    }

    #[inline]
    #[must_use]
    fn head(&self) -> State {
        self.kv().head()
    }
}

impl AccountKVStore {
    /// Create a new `AccountStore` instance for `Address` `account_addr`.
    ///
    /// Delegates work to underlying key-value store `kv`.
    pub fn new(account_addr: Address, kv: &Arc<Mutex<dyn StatefulKV + Send>>) -> Self {
        let kv = Arc::clone(&kv);

        Self { account_addr, kv }
    }

    #[inline]
    fn build_key(&self, key: &[u8]) -> Vec<u8> {
        debug_assert_eq!(key.len(), 4);

        let mut buf = Vec::with_capacity(Address::len() + key.len());

        buf.extend_from_slice(self.account_addr.as_slice());
        buf.extend_from_slice(key);

        self.hash(&buf)
    }

    #[inline]
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        Blake3Hasher::hash(bytes).to_vec()
    }

    fn kv(&self) -> MutexGuard<dyn StatefulKV + Send + 'static> {
        self.kv.try_lock().unwrap()
    }
}

impl Clone for AccountKVStore {
    fn clone(&self) -> Self {
        Self {
            account_addr: self.account_addr.clone(),
            kv: Arc::clone(&self.kv),
        }
    }
}
