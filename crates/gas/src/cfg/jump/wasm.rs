use std::fmt::{self, Debug};

use parity_wasm::elements::Instruction;

#[derive(Clone, PartialEq)]
pub enum WasmJump {
    BrIf(u32),
    Br(u32),
    BrTable(Box<WasmBrTable>),
    Return,
    Unreachable,
}

#[derive(Clone, PartialEq)]
pub struct WasmBrTable {
    default: u32,

    labels: Box<[u32]>,
}

impl WasmBrTable {
    pub fn default(&self) -> u32 {
        self.default
    }

    pub fn iter(&self) -> core::slice::Iter<'_, u32> {
        self.labels.iter()
    }
}

impl From<&Instruction> for WasmJump {
    fn from(op: &Instruction) -> Self {
        match *op {
            Instruction::Return => WasmJump::Return,
            Instruction::Unreachable => WasmJump::Unreachable,
            Instruction::Br(index) => WasmJump::Br(index),
            Instruction::BrIf(index) => WasmJump::BrIf(index),
            Instruction::BrTable(ref table) => {
                let table = WasmBrTable {
                    default: table.default,
                    labels: table.table.clone(),
                };

                WasmJump::BrTable(Box::new(table))
            }
            _ => unreachable!(),
        }
    }
}

impl Debug for WasmJump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            WasmJump::Return => write!(f, "return"),
            WasmJump::Unreachable => write!(f, "unreachable"),
            WasmJump::Br(n) => write!(f, "br {}", n),
            WasmJump::BrIf(n) => write!(f, "br_if {}", n),
            WasmJump::BrTable(ref table) => {
                let default = table.default;

                let mut table_buf = String::new();

                for s in table.labels.iter().map(|n| n.to_string()) {
                    table_buf.push_str(&s);
                    table_buf.push(' ');
                }

                write!(f, "br_table {} {}", table_buf, default)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use parity_wasm::elements::BrTableData;

    #[test]
    fn wasm_return_convert() {
        let op = Instruction::Return;

        assert_eq!(WasmJump::from(&op), WasmJump::Return);
    }

    #[test]
    fn wasm_unreachable_convert() {
        let op = Instruction::Unreachable;

        assert_eq!(WasmJump::from(&op), WasmJump::Unreachable);
    }

    #[test]
    fn wasm_br_convert() {
        let op = Instruction::Br(10);

        assert_eq!(WasmJump::from(&op), WasmJump::Br(10));
    }

    #[test]
    fn wasm_br_if_convert() {
        let op = Instruction::BrIf(10);

        assert_eq!(WasmJump::from(&op), WasmJump::BrIf(10));
    }

    #[test]
    fn wasm_br_table_convert() {
        let default = 5;
        let table = vec![10, 20, 30].into_boxed_slice();

        let data = BrTableData {
            table: table.clone(),
            default,
        };

        let op = Instruction::BrTable(Box::new(data));
        let data = WasmBrTable {
            labels: table,
            default,
        };

        assert_eq!(WasmJump::from(&op), WasmJump::BrTable(Box::new(data)));
    }
}
