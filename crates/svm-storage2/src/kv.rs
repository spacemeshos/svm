pub trait KV {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    fn set(&mut self, changes: &[(&[u32], &[u8])]);
}
