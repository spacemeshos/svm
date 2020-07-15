#[derive(Debug, PartialEq, Clone)]
pub struct Log {
    pub msg: Vec<u8>,

    pub code: u8,
}
