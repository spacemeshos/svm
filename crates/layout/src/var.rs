/// Represents a raw variable. an unsigned integer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Id(pub u32);

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
    name: String,

    ty: Type,
}

impl SymbolicVar {
    pub fn new(name: String, ty: Type) -> Self {
        Self { name, ty }
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
