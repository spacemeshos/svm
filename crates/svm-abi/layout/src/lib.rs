//!
//! +-----------+-------------------------------------+
//! |   Nibble  | Nibble  |        Meaning            |
//! +-----------+---------+---------------------------+
//! | 0 | 0 0 0 | 0 0 0 0 |  False (Boolean)          |
//! | 0 | 0 0 1 | 0 0 0 0 |  True  (Boolean)          |
//! | 0 | 0 1 0 | 0 0 0 0 |  Array Start              |
//! | 0 | 0 1 1 | 0 0 0 0 |  Array End                |
//! | 0 | 1 0 0 | 0 0 0 0 |  Tuple Start (Reserved)   |
//! | 0 | 1 0 1 | 0 0 0 0 |  Tuple End   (Reserved)   |
//! | 0 | 1 1 0 | 0 0 0 0 |  Address                  |
//! | 0 | 1 1 1 | 0 0 0 0 |  String (Reserved)        |
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
//! | 0 | 1 0 1 | 0 0 1 0 |  i16 (signed)   - 2 bytes |
//! | 0 | 1 1 0 | 0 0 1 0 |  Hash (Reserved)          |
//! | 0 | 1 1 1 | 0 0 1 0 |  Blob (Reserved)          |
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
//!
//!
//!

#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

pub mod layout {
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

    /// Amount
    pub const AMOUNT_1B: u8 = 0b_0000_0000;
    pub const AMOUNT_2B: u8 = 0b_0000_0000;
    pub const AMOUNT_3B: u8 = 0b_0000_0000;
    pub const AMOUNT_4B: u8 = 0b_0000_0000;
    pub const AMOUNT_5B: u8 = 0b_0000_0000;
    pub const AMOUNT_6B: u8 = 0b_0000_0000;
    pub const AMOUNT_7B: u8 = 0b_0000_0000;
    pub const AMOUNT_8B: u8 = 0b_0000_0000;

    // i8
    //// signed
    pub const I8: u8 = 0b_0000_0000;
    /// unsigned
    pub const U8: u8 = 0b_0000_0000;

    // i16
    //// signed
    pub const I16_1B: u8 = 0b_0000_0000;
    pub const I16_2B: u8 = 0b_0000_0000;
    //// unsigned
    pub const U16_1B: u8 = 0b_0000_0000;
    pub const U16_2B: u8 = 0b_0000_0000;

    /// Hash (Reserved)
    pub const HASH: u8 = 0b_0000_0000;

    /// Blob (Reserved)
    pub const BLOB: u8 = 0b_0000_0000;

    // i32
    //// signed
    pub const I32_1B: u8 = 0b_0000_0000;
    pub const I32_2B: u8 = 0b_0000_0000;
    pub const I32_3B: u8 = 0b_0000_0000;
    pub const I32_4B: u8 = 0b_0000_0000;
    //// unsigned
    pub const U32_1B: u8 = 0b_0000_0000;
    pub const U32_2B: u8 = 0b_0000_0000;
    pub const U32_3B: u8 = 0b_0000_0000;
    pub const U32_4B: u8 = 0b_0000_0000;

    // i64
    //// signed
    pub const I64_1B: u8 = 0b_0000_0000;
    pub const I64_2B: u8 = 0b_0000_0000;
    pub const I64_3B: u8 = 0b_0000_0000;
    pub const I64_4B: u8 = 0b_0000_0000;
    pub const I64_5B: u8 = 0b_0000_0000;
    pub const I64_6B: u8 = 0b_0000_0000;
    pub const I64_7B: u8 = 0b_0000_0000;
    pub const I64_8B: u8 = 0b_0000_0000;
    //// unsigned
    pub const U64_1B: u8 = 0b_0000_0000;
    pub const U64_2B: u8 = 0b_0000_0000;
    pub const U64_3B: u8 = 0b_0000_0000;
    pub const U64_4B: u8 = 0b_0000_0000;
    pub const U64_5B: u8 = 0b_0000_0000;
    pub const U64_6B: u8 = 0b_0000_0000;
    pub const U64_7B: u8 = 0b_0000_0000;
    pub const U64_8B: u8 = 0b_0000_0000;
}
