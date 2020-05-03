pub trait KV {
    fn get(&self) -> Option<Vec<u8>>;

    fn set(&mut self, changes: &[(&[u8], &[u8])]);
}
