//!
//! +-----------+-------------------------------------+
//! |   Nibble  | Nibble  |        Meaning            |
//! +-----------+---------+---------------------------+
//! | 0 | 0 0 0 | 0 0 0 0 |  False (Boolean)          |
//! | 0 | 0 0 1 | 0 0 0 0 |  True  (Boolean)          |
//! | 0 | 0 1 0 | 0 0 0 0 |  None                     |
//! | 0 | 0 1 1 | 0 0 0 0 |  Unit                     |
//! | 0 | 1 0 0 | 0 0 0 0 |  Address                  |
//! | 0 | 1 0 1 | 0 0 0 0 |  Reserved                 |
//! | 0 | 1 1 0 | 0 0 0 0 |  Reserved                 |
//! | 0 | 1 1 1 | 0 0 0 0 |  Reserved                 |
//! +---+-------+---------+---------------------------+
//! | 0 | 0 0 0 | 0 0 0 1 |  Amount - 1 byte          |
//! | 0 | 0 0 1 | 0 0 0 1 |  Amount - 2 bytes         |
//! | 0 | 0 1 0 | 0 0 0 1 |  Amount - 3 bytes         |
//! | 0 | 0 1 1 | 0 0 0 1 |  Amount - 4 bytes         |
//! | 0 | 1 0 0 | 0 0 0 1 |  Amount - 5 bytes         |
//! | 0 | 1 0 1 | 0 0 0 1 |  Amount - 6 bytes         |
//! | 0 | 1 1 0 | 0 0 0 1 |  Amount - 7 bytes         |
//! | 0 | 1 1 1 | 0 0 0 1 |  Amount - 8 bytes         |
//! +---+-------+---------+---------------------------+
//! | 0 | 0 0 0 | 0 0 1 0 |  i8  (signed)             |
//! | 0 | 0 0 1 | 0 0 1 0 |  u8  (unsigned)           |
//! | 0 | 0 1 0 | 0 0 1 0 |  i16 (signed)   - 1 byte  |
//! | 0 | 0 1 1 | 0 0 1 0 |  i16 (signed)   - 2 bytes |
//! | 0 | 1 0 0 | 0 0 1 0 |  u16 (unsigned) - 1 byte  |
//! | 0 | 1 0 1 | 0 0 1 0 |  u16 (signed)   - 2 bytes |
//! | 0 | 1 1 0 | 0 0 1 0 |  Reserved                 |
//! | 0 | 1 1 1 | 0 0 1 0 |  Reserved                 |
//! +---+-------+---------+---------------------------+
//! | 0 | 0 0 0 | 0 0 1 1 |  i32 (signed)   - 1 byte  |
//! | 0 | 0 0 1 | 0 0 1 1 |  i32 (signed)   - 2 bytes |
//! | 0 | 0 1 0 | 0 0 1 1 |  i32 (signed)   - 3 bytes |
//! | 0 | 0 1 1 | 0 0 1 1 |  i32 (signed)   - 4 bytes |
//! | 0 | 1 0 0 | 0 0 1 1 |  i32 (unsigned) - 1 bytes |
//! | 0 | 1 0 1 | 0 0 1 1 |  i32 (unsigned) - 2 bytes |
//! | 0 | 1 1 0 | 0 0 1 1 |  i32 (unsigned) - 3 bytes |
//! | 0 | 1 1 1 | 0 0 1 1 |  i32 (unsigned) - 4 bytes |
//! +---+-------+---------+---------------------------+
//! | 0 | 0 0 0 | 0 1 0 0 |  i64 (signed)   - 1 byte  |
//! | 0 | 0 0 1 | 0 1 0 0 |  i64 (signed)   - 2 bytes |
//! | 0 | 0 1 0 | 0 1 0 0 |  i64 (signed)   - 3 bytes |
//! | 0 | 0 1 1 | 0 1 0 0 |  i64 (signed)   - 4 bytes |
//! | 0 | 1 0 0 | 0 1 0 0 |  i64 (signed)   - 5 bytes |
//! | 0 | 1 0 1 | 0 1 0 0 |  i64 (signed)   - 6 bytes |
//! | 0 | 1 1 0 | 0 1 0 0 |  i64 (signed)   - 7 bytes |
//! | 0 | 1 1 1 | 0 1 0 0 |  i64 (signed)   - 8 bytes |
//! +---+-------+---------+---------------------------+
//! | 0 | 0 0 0 | 0 1 0 1 |  i64 (unsigned) - 1 byte  |
//! | 0 | 0 0 1 | 0 1 0 1 |  i64 (unsigned) - 2 bytes |
//! | 0 | 0 1 0 | 0 1 0 1 |  i64 (unsigned) - 3 bytes |
//! | 0 | 0 1 1 | 0 1 0 1 |  i64 (unsigned) - 4 bytes |
//! | 0 | 1 0 0 | 0 1 0 1 |  i64 (unsigned) - 5 bytes |
//! | 0 | 1 0 1 | 0 1 0 1 |  i64 (unsigned) - 6 bytes |
//! | 0 | 1 1 0 | 0 1 0 1 |  i64 (unsigned) - 7 bytes |
//! | 0 | 1 1 1 | 0 1 0 1 |  i64 (unsigned) - 8 bytes |
//! +---+-------+---------+---------------------------+
//! | 0 | 0 0 0 | 0 1 1 0 |  Array - 0  items         | <------ Small-Array (at most 10 items inclusive)
//! | 0 | 0 0 1 | 0 1 1 0 |  Array - 1  item          |
//! | 0 | 0 1 0 | 0 1 1 0 |  Array - 2  items         |
//! | 0 | 0 1 1 | 0 1 1 0 |  Array - 3  items         |
//! | 0 | 1 0 0 | 0 1 1 0 |  Array - 4  items         |
//! | 0 | 1 0 1 | 0 1 1 0 |  Array - 5  items         |
//! | 0 | 1 1 0 | 0 1 1 0 |  Array - 6  items         |
//! | 0 | 1 1 1 | 0 1 1 0 |  Array - 7  items         |
//! | 0 | 0 0 0 | 0 1 1 1 |  Array - 8  items         |
//! | 0 | 0 0 1 | 0 1 1 1 |  Array - 9  items         |
//! | 0 | 0 1 0 | 0 1 1 1 |  Array - 10 items         |
//! +---+-----------------+---------------------------+
//!

#![no_std]
#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::unusual_byte_groupings)]
#![allow(missing_docs)]

pub const BOOL_FALSE: u8 = 0b_0_000_0000;
pub const BOOL_TRUE: u8 = 0b_0_001_0000;

pub const NONE: u8 = 0b_0_010_0000;

pub const UNIT: u8 = 0b_0_011_0000;

pub const ADDRESS: u8 = 0b_0_100_0000;

pub const AMOUNT_1B: u8 = 0b_0_000_0001;
pub const AMOUNT_2B: u8 = 0b_0_001_0001;
pub const AMOUNT_3B: u8 = 0b_0_010_0001;
pub const AMOUNT_4B: u8 = 0b_0_011_0001;
pub const AMOUNT_5B: u8 = 0b_0_100_0001;
pub const AMOUNT_6B: u8 = 0b_0_101_0001;
pub const AMOUNT_7B: u8 = 0b_0_110_0001;
pub const AMOUNT_8B: u8 = 0b_0_111_0001;

pub const I8: u8 = 0b_0_000_0010;
pub const U8: u8 = 0b_0_001_0010;

pub const I16_1B: u8 = 0b_0_010_0010;
pub const I16_2B: u8 = 0b_0_011_0010;
pub const U16_1B: u8 = 0b_0_100_0010;
pub const U16_2B: u8 = 0b_0_101_0010;

pub const I32_1B: u8 = 0b_0_000_0011;
pub const I32_2B: u8 = 0b_0_001_0011;
pub const I32_3B: u8 = 0b_0_010_0011;
pub const I32_4B: u8 = 0b_0_011_0011;
pub const U32_1B: u8 = 0b_0_100_0011;
pub const U32_2B: u8 = 0b_0_101_0011;
pub const U32_3B: u8 = 0b_0_110_0011;
pub const U32_4B: u8 = 0b_0_111_0011;

pub const I64_1B: u8 = 0b_0_000_0100;
pub const I64_2B: u8 = 0b_0_001_0100;
pub const I64_3B: u8 = 0b_0_010_0100;
pub const I64_4B: u8 = 0b_0_011_0100;
pub const I64_5B: u8 = 0b_0_100_0100;
pub const I64_6B: u8 = 0b_0_101_0100;
pub const I64_7B: u8 = 0b_0_110_0100;
pub const I64_8B: u8 = 0b_0_111_0100;
pub const U64_1B: u8 = 0b_0_000_0101;
pub const U64_2B: u8 = 0b_0_001_0101;
pub const U64_3B: u8 = 0b_0_010_0101;
pub const U64_4B: u8 = 0b_0_011_0101;
pub const U64_5B: u8 = 0b_0_100_0101;
pub const U64_6B: u8 = 0b_0_101_0101;
pub const U64_7B: u8 = 0b_0_110_0101;
pub const U64_8B: u8 = 0b_0_111_0101;

// Small arrays.
pub const ARR_0: u8 = 0b_0_000_0110;
pub const ARR_1: u8 = 0b_0_001_0110;
pub const ARR_2: u8 = 0b_0_010_0110;
pub const ARR_3: u8 = 0b_0_011_0110;
pub const ARR_4: u8 = 0b_0_100_0110;
pub const ARR_5: u8 = 0b_0_101_0110;
pub const ARR_6: u8 = 0b_0_110_0110;
pub const ARR_7: u8 = 0b_0_111_0110;
pub const ARR_8: u8 = 0b_0_000_0111;
pub const ARR_9: u8 = 0b_0_001_0111;
pub const ARR_10: u8 = 0b_0_010_0111;
