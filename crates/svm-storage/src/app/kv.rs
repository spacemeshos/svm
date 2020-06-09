use std::cell::RefCell;
use std::rc::Rc;

use crate::kv::StatefulKV;

use svm_common::{Address, DefaultKeyHasher, KeyHasher, State};

/// An application-aware (and `State`-aware) key-value store interface responsible of
/// mapping `u32` input keys (given as a 4 byte-length slice) to global keys under a raw key-value store.
///
/// The mapping is dependant on the contextual app's `Address` (see the `new` method).
pub struct AppKVStore {
    pub(crate) app_addr: Address,

    pub(crate) kv: Rc<RefCell<dyn StatefulKV>>,
}

impl StatefulKV for AppKVStore {
    #[inline]
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let key = self.build_key(key);

        self.kv.borrow().get(&key)
    }

    #[inline]
    fn set(&mut self, key: &[u8], value: &[u8]) {
        let key = self.build_key(key);

        self.kv.borrow_mut().set(&key, value);
    }

    #[inline]
    fn discard(&mut self) {
        self.kv.borrow_mut().discard();
    }

    #[inline]
    fn flush(&mut self) {
        self.kv.borrow_mut().flush();
    }

    #[inline]
    #[must_use]
    fn checkpoint(&mut self) -> State {
        self.kv.borrow_mut().checkpoint()
    }

    #[inline]
    #[must_use]
    fn rewind(&mut self, state: &State) {
        self.kv.borrow_mut().rewind(state);
    }

    #[inline]
    #[must_use]
    fn head(&self) -> State {
        self.kv.borrow().head()
    }
}

impl AppKVStore {
    /// Create a new `AppKVStore` instance for application `app_addr`.
    ///
    /// Delegates work to raw key-value store `kv`.
    pub fn new(app_addr: Address, kv: &Rc<RefCell<dyn StatefulKV>>) -> Self {
        let kv = Rc::clone(&kv);

        Self { app_addr, kv }
    }

    #[inline]
    fn build_key(&self, key: &[u8]) -> Vec<u8> {
        debug_assert_eq!(key.len(), 4);

        let mut buf = Vec::with_capacity(Address::len() + key.len());

        buf.extend_from_slice(self.app_addr.as_slice());
        buf.extend_from_slice(key);

        self.hash(&buf)
    }

    #[inline]
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        DefaultKeyHasher::hash(bytes).to_vec()
    }
}

impl Clone for AppKVStore {
    fn clone(&self) -> Self {
        Self {
            app_addr: self.app_addr.clone(),
            kv: Rc::clone(&self.kv),
        }
    }
}
