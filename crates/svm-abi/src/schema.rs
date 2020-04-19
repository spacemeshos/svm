use std::collections::HashMap;

///  Schema's variable representation
#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    /// Var's unique id
    pub id: usize,

    /// Variable's layout.
    pub layout: VarLayout,

    /// Variable's type.
    pub ty: VarType,

    /// Variable's symbolic name.
    pub name: String,

    /// Variable's description (a free-text documentation).
    pub desc: String,
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
    Bool,

    /// `Public-Key`
    PubKey,

    /// Account's Address
    Address,
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
pub struct Schema(HashMap<usize, Var>);

impl Schema {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the variable's schema data
    pub fn get_var(&self, id: usize) -> Option<Var> {
        self.0.get(&id).cloned()
    }
}
