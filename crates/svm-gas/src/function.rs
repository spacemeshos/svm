use parity_wasm::elements::{Instruction, Instructions};

/// Represents a function index
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

#[derive(Debug)]
pub struct FuncBody(pub Instructions);

impl FuncBody {
    pub fn to_vec(&self) -> Vec<Instruction> {
        self.0.elements().to_vec()
    }
}
