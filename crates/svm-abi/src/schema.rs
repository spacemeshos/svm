use std::collections::HashMap;

///  Schema's variable representation
#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    /// Var's unique id
    id: usize,

    /// Variable's layout.
    layout: VarLayout,

    /// Variable's type.
    ty: VarType,

    /// Variable's symbolic name.
    name: String,

    /// Variable's description (a free-text documentation).
    desc: String,

    /// Variable's value.
    value: Vec<u8>,
}

/// Variables storage layout
#[derive(Debug, Clone, PartialEq)]
pub struct VarLayout {
    /// Page index that holds the variable.
    page_idx: usize,

    /// Starting offset within page.
    offset: usize,

    /// Byte length of the variable.
    length: usize,
}

/// Variable type.
#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    /// `Integer` with Endianness (Big-Endian / Litte-Endian).
    Int(Endianness),

    /// Blob of data
    Blob,

    /// UTF-8 String
    String,

    /// Represents an Account's balance. (non-negative Big-Endian Integer)
    Balance,

    /// A boolean (True / False)
    Boolean,

    /// `Public-Key`
    PubKey,

    /// Account's Address
    Address,

    /// Represents `Hash` blob of data
    Hash,
    // Struct(Box<Vec<Var>>),
}

/// Integer Endianness
#[derive(Debug, Clone, PartialEq)]
pub enum Endianness {
    /// Big-Endian
    Big,

    /// Little-Endian
    Little,
}

/// Holds a representation of `AppStorage`'s schema.
#[derive(Debug, Clone, PartialEq)]
pub struct AppSchema(HashMap<usize, Var>);

impl AppSchema {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the variable's schema data
    pub fn get_var(&self, id: usize) -> Option<Var> {
        self.0.get(&id).cloned()
    }
}
