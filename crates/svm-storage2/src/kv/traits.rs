#[doc(hidden)]
pub trait KV {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    fn set(&mut self, changes: &[(Vec<u8>, Vec<u8>)]);
}
