use std::ops::{self, Add, AddAssign};

/// A type alias for SVM contract raw variables.
pub type Id = u32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RawVar {
    pub id: Id,
    pub offset: u32,
    pub byte_size: u32,
}

impl RawVar {
    pub fn new(id: Id, offset: u32, byte_size: u32) -> Self {
        Self {
            id,
            offset,
            byte_size,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolicVar {
    pub id: Id,
    pub name: String,
    pub ty: Type,
}

impl SymbolicVar {
    pub fn new(id: Id, name: String, ty: Type) -> Self {
        Self { id, name, ty }
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
