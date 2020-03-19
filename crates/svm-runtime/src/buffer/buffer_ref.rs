pub use crate::buffer::{Buffer, BufferMut};

/// Owns a buffer of type read-only (`Buffer`) or of read-write (`BufferMut`).
pub enum BufferRef {
    /// Read-Only buffer associated with its index.
    ReadOnly(u32, Buffer),

    /// Read/Write buffer associated with its index.
    Mutable(u32, BufferMut),
}

impl BufferRef {
    /// Returns the the buffer byte-length.
    pub fn len(&self) -> u32 {
        match self {
            BufferRef::ReadOnly(.., buf) => buf.len(),
            BufferRef::Mutable(.., buf) => buf.len(),
        }
    }

    /// Returns a slice to buffer underlying bytes `offset, offset + 1, ..., offset + len - 1`
    pub fn read(&self, offset: u32, len: u32) -> &[u8] {
        match self {
            BufferRef::ReadOnly(.., buf) => buf.read(offset, len),
            BufferRef::Mutable(.., buf) => buf.read(offset, len),
        }
    }

    /// Appends `slice` into buffer data.
    //  Panics if owned buffer is of type read-only.
    pub fn write(&mut self, slice: &[u8]) {
        match self {
            BufferRef::Mutable(.., buf) => buf.write(slice),
            BufferRef::ReadOnly(buf_id, ..) => panic!(format!("Buffer `{}` is read-only!", buf_id)),
        }
    }

    /// Turns buffer into a read-only one.
    /// Does nothing it case the buffer is already read-only.
    pub fn freeze(self) -> BufferRef {
        match self {
            BufferRef::ReadOnly(..) => self,
            BufferRef::Mutable(buf_id, buf) => {
                let buf = BufferMut::freeze(buf);

                BufferRef::ReadOnly(buf_id, buf)
            }
        }
    }
}
