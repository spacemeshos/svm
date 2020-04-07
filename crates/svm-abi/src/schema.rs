///  Schema's variable representation
#[derive(Debug, Clone, Hash)]
pub struct Var {
    /// Variable's layout
    layout: VarLayout,

    /// Variable's type
    ty: VarType,

    /// Variable's symbolic name
    name: String,

    /// Variable's description (a free-text documentation).
    desc: String,
}

/// Variables storage layout
#[derive(Debug, Clone, Hash)]
pub struct VarLayout {
    /// Page index that holds the variable.
    page_idx: usize,

    /// Starting offset within page.
    offset: usize,

    /// Byte length of the variable.
    length: usize,
}

/// Variable type.
#[derive(Debug, Clone, Hash)]
pub enum VarType {
    /// `Integer` with Endianness (BigEndian / LitteEndian).
    Int(Endianness),

    /// Blob of data
    Blob,

    /// UTF-8 String
    String,

    /// Represents an Account's balance. (non-negative Big-Endian Integer)
    Balance,

    /// A boolean (True / False)
    Boolean,

    /// `Public-Key`.
    PubKey,

    /// Account's Address
    Address,

    /// Represents `Hash` blob of data
    Hash,
    // Struct(Box<Vec<Var>>),
}

/// Integer Endianness
#[derive(Debug, Copy, Clone, Hash)]
pub enum Endianness {
    /// Big-Endian
    Big,

    /// Little-Endian
    Little,
}

/// Holds a representation of `AppStorage`'s schema.
#[derive(Debug, Clone, Hash)]
pub struct AppSchema(Vec<Var>);

impl AppSchema {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self(Vec::new())
    }
}
