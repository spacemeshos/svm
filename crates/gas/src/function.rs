use crate::Gas;
use std::collections::HashMap;

use parity_wasm::elements::{Instruction, Instructions};

/// Represents a function index
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct FuncIndex(pub u16);

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

#[derive(Debug)]
pub(crate) struct FuncsGas {
    pub inner: HashMap<FuncIndex, Gas>,
}

impl FuncsGas {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn set_func_gas(&mut self, func_idx: FuncIndex, gas: Gas) {
        self.inner.insert(func_idx, gas);
    }

    pub fn get_func_gas(&self, func_idx: FuncIndex) -> Option<Gas> {
        self.inner.get(&func_idx).copied()
    }
}
