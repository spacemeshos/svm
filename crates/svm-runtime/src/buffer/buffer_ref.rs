pub use crate::buffer::{Buffer, BufferMut};

pub enum BufferRef {
    ReadOnly(u32, Buffer),

    Mutable(u32, BufferMut),
}

impl BufferRef {
    pub fn len(&self) -> u32 {
        match self {
            BufferRef::ReadOnly(.., buf) => buf.len(),
            BufferRef::Mutable(.., buf) => buf.len(),
        }
    }

    pub fn read(&self, offset: u32, len: u32) -> &[u8] {
        match self {
            BufferRef::ReadOnly(.., buf) => buf.read(offset, len),
            BufferRef::Mutable(.., buf) => buf.read(offset, len),
        }
    }

    pub fn write(&mut self, slice: &[u8]) {
        match self {
            BufferRef::Mutable(.., buf) => buf.write(slice),
            BufferRef::ReadOnly(buf_id, ..) => panic!(format!("Buffer `{}` is read-only!", buf_id)),
        }
    }

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
