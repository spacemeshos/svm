use std::fmt::{self, Debug, Display};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct FuncIndex(pub u32);

impl PartialOrd for FuncIndex {
    fn partial_cmp(&self, rhs: &FuncIndex) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl Ord for FuncIndex {
    fn cmp(&self, rhs: &FuncIndex) -> std::cmp::Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl Display for FuncIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
