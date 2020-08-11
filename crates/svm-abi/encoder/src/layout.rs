//
// +---------+------------------------------------+
// | Nibble  | Nibble  |  Meaning                 |
// +---------+------------------------------------+
// | 0 0 0 0 |---------|  False (Boolean)         |
// | 0 0 0 1 |---------|  True  (Boolean)         |
// | 0 0 1 0 |---------|  Array Start             |
// | 0 0 1 1 |---------|  Array End               |
// | 0 1 0 0 |---------|  Tuple Start (Reserved)  |
// | 0 1 0 1 |---------|  Tuple End   (Reserved)  |
// | 0 1 1 0 |---------|  Address                 |
// | 0 1 1 1 |---------|  String (Reserved)       |
// +---------+---------+--------------------------+
// | 1 0 0 0 | 0 0 0 0 |  i32 - 0 bytes           |
// | 1 0 0 1 | 0 0 0 0 |  i32 - 1 bytes           |
// | 1 0 1 0 | 0 0 0 0 |  i32 - 2 bytes           |
// | 1 0 1 1 | 0 0 0 0 |  i32 - 3 bytes           |
// | 1 1 0 0 | 0 0 0 0 |  i32 - 4 bytes           |
// | 1 1 0 1 | 0 0 0 0 |  i64 - 0 bytes           |
// | 1 1 1 0 | 0 0 0 0 |  i64 - 1 byte            |
// | 1 1 1 1 | 0 0 0 0 |  i64 - 2 bytes           |
// +---------+------------------------------------+
// | 1 0 0 0 | 0 0 0 1 |  i64 - 3 bytes           |
// | 1 0 0 1 | 0 0 0 1 |  i64 - 4 bytes           |
// | 1 0 1 0 | 0 0 0 1 |  i64 - 5 bytes           |
// | 1 0 1 1 | 0 0 0 1 |  i64 - 6 bytes           |
// | 1 1 0 0 | 0 0 0 1 |  i64 - 7 bytes           |
// | 1 1 0 1 | 0 0 0 1 |  i64 - 8 bytes           |
// | 1 1 1 0 | 0 0 0 1 |  Hash (Reserved)         |
// | 1 1 1 1 | 0 0 0 1 |  Blob (Reserved)         |
// +---------+---------+--------------------------+
//

use svm_nibble::{nib, Nibble};
use svm_sdk::types::Primitive as Type;
use svm_sdk::value::Primitive as Value;

// Boolean
pub const BOOL_FALSE: u8 = 0b_0000_0000;
pub const BOOL_TRUE: u8 = 0b_0000_0001;

// Array
pub const ARRAY_START: u8 = 0b_0000_0010;
pub const ARRAY_END: u8 = 0b_0000_0011;

// Tuple
pub const TUPLE_START: u8 = 0b_0000_0100;
pub const TUPLE_END: u8 = 0b_0000_0101;

// Address
pub const ADDRESS: u8 = 0b_0000_0110;

// String (Reserved)
pub const STRING: u8 = 0b_0000_0111;

// i32
pub const I32_0B: u8 = 0b_1000_0000;
pub const I32_1B: u8 = 0b_1001_0000;
pub const I32_2B: u8 = 0b_1010_0000;
pub const I32_3B: u8 = 0b_1011_0000;
pub const I32_4B: u8 = 0b_1100_0000;

// i64
pub const I64_0B: u8 = 0b_1101_0000;
pub const I64_1B: u8 = 0b_1110_0000;
pub const I64_2B: u8 = 0b_1111_0000;
pub const I64_3B: u8 = 0b_1000_0001;
pub const I64_4B: u8 = 0b_1001_0001;
pub const I64_5B: u8 = 0b_1010_0001;
pub const I64_6B: u8 = 0b_1011_0001;
pub const I64_7B: u8 = 0b_1100_0001;
pub const I64_8B: u8 = 0b_1101_0001;

/// Hash (Reserved)
pub const HASH: u8 = 0b_1110_0001;

/// Blob (Reserved)
pub const BLOB: u8 = 0b_1111_0001;

#[derive(Debug, Clone, PartialEq)]
struct ValueLayout {
    pub ty: Type,

    pub len: usize,
}

pub fn value_layout(value: &Value) -> ValueLayout {
    match value {
        Value::I32(v) => {
            let len = value_byte_length(*v as u64);
            debug_assert!(len <= 4);

            ValueLayout { ty: Type::I32, len }
        }
        Value::I64(v) => {
            let len = wasm_value_byte_length(*v);

            debug_assert!(len <= 8);

            ValueLayout { ty: Type::I64, len }
        }
    }
}

fn value_byte_length(value: u64) -> usize {
    match value {
        0 => 0,
        0x01..=0xFF => 1,
        0x01_00..=0xFF_FF => 2,
        0x_01_00_00..=0xFF_FF_FF => 3,
        0x_01_00_00_00..=0xFF_FF_FF_FF => 4,
        0x_01_00_00_00_00..=0xFF_FF_FF_FF_FF => 5,
        0x_01_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF => 6,
        0x_01_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF => 7,
        0x_01_00_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF_FF => 8,
    }
}

impl Into<Nibble> for &ValueLayout {
    fn into(self) -> Nibble {
        let byte = {
            match (self.ty, self.len) {
                // i32
                (Type::I32, 0) => I32_0B,
                (Type::I32, 1) => I32_1B,
                (Type::I32, 2) => I32_2B,
                (Type::I32, 3) => I32_3B,
                (Type::I32, 4) => I32_4B,
                //
                // i64
                (Type::I64, 0) => I64_0B,
                (Type::I64, 1) => I64_1B,
                (Type::I64, 2) => I64_2B,
                (Type::I64, 3) => I64_3B,
                (Type::I64, 4) => I64_4B,
                (Type::I64, 5) => I64_5B,
                (Type::I64, 6) => I64_6B,
                (Type::I64, 7) => I64_7B,
                (Type::I64, 8) => I64_8B,
                _ => unreachable!(),
            }
        };

        nib!(byte)
    }
}
