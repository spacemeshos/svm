#[derive(Debug, Clone, PartialEq)]
pub struct BufferSlice {
    buf_id: i32,
    offset: i32,
    data: Vec<u8>,
}
