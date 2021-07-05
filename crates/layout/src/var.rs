use std::ops::{self, Add, AddAssign};

/// Represents a raw variable. an unsigned integer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id(pub u32);

impl Add<u32> for Id {
    type Output = Id;

    fn add(self, rhs: u32) -> Self::Output {
        let n = self.0.checked_add(rhs).unwrap();

        Id(n)
    }
}

impl AddAssign<u32> for Id {
    fn add_assign(&mut self, rhs: u32) {
        self.0 = self.0.checked_add(rhs).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawVar {
    id: Id,

    offset: u32,

    byte_size: u32,
}

impl RawVar {
    pub fn new(id: Id, offset: u32, byte_size: u32) -> Self {
        Self {
            id,
            offset,
            byte_size,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn byte_size(&self) -> u32 {
        self.byte_size
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolicVar {
    id: Id,

    name: String,

    ty: Type,
}

impl SymbolicVar {
    pub fn new(id: Id, name: String, ty: Type) -> Self {
        Self { id, name, ty }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Primitive {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Amount,
    Address,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(Primitive),

    Array { primitive: Primitive, length: usize },
}
