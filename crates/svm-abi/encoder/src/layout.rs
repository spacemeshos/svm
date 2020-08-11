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
