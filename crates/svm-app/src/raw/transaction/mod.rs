//!     Execute `AppTransaction` Raw Format Version 0.0
//!  -------------------------------------------------------
//!  |   proto     |                                       |
//!  |  version    |            `AppAddress`               |
//!  |  encoding   |             (20 bytes)                |
//!  |    (a)      |                                       |
//!  |_____________|_______________________________________|
//!  |                                                     |
//!  |           Function Index Encoding (b)               |
//!  |_____________________________________________________|
//!  |            |                                        |
//!  |  func-buf  |                                        |
//!  |  #length   |           func-buf blob                |
//!  |  encoding  |                                        |
//!  |    (c)     |                                        |
//!  |____________|________________________________________|
//!  |   func    |   func    |   func    |                 |
//!  |  arg #1   |  arg #2   |  arg #3   |                 |
//!  |  layout   |  layout   |  layout   |     . . . .     |
//!  | encoding  | encoding  | encoding  |                 |
//!  |   (d)     |           |           |                 |
//!  |___________|___________|___________|_________________|
//!  |   func    |   func    |   func    |                 |
//!  |  arg #1   |  arg #2   |  arg #3   |     . . . .     |                 
//!  |   value   |   value   |   value   |                 |
//!  |___________|___________|___________|_________________|
//!
//!
//!
//! (a) Proto Version Encoding
//! ===========================
//!     MSB     non-MSB           Meaning
//!  ---------------------------------------------------
//!  |   1   |  x  x  x  |  Next nibble is relevant too |
//!  |   0   |  x  x  x  |  Next nibble isn't relevant  |
//!  |--------------------------------------------------|
//!
//!  The protocol `#bits` will be a multiplication of 3,
//!  and is encoded in Little-endian as an unsigned-integer.
//!
//!  
//!  
//! (b) Function Index Encoding  
//! ============================
//!
//!  Number is represented in Little-Endian layout (an unsigned-integer).
//!
//!     MSBs    Rest                   Meaning      
//!  -----------------------------------------------------------
//!  |  0 0   |  .  .  |  Index takes 1 nibble  (2 used bits)  |  
//!  |  0 1   |  .  .  |  Index takes 2 nibbles (6 used bits)  |  
//!  |  1 0   |  .  .  |  Index takes 3 nibbles (10 used bits) |   
//!  |  1 1   |  .  .  |  Index takes 4 nibbles (14 uses bits) |
//!  |--------|--------|---------------------------------------|
//!
//!  The `#bits` used numbers are `2 / 6 / 10 / 14` since the two MSB bits
//!  of the 1st nibble tell us how many nibbles are part of the encoding.
//!  
//!  So if we need 4 nibbles for representing the function index,
//!  The first nibble will donate 2 bits and the other 3 nibbles will donate 4 bits each.
//!  So we get: 2 + 3 * 4 = 14
//!  
//!
//!  
//! (c) Function Buf Length Encoding
//! ================================
//! The same encoding as `Function Index Encoding` (b)
//!
//!
//!
//! (d) Func Args Layout Encoding
//! =============================
//!
//!  Each function arg is represented by one nibble.
//!
//!  i32/i64 | #bytes  |           Meaning
//!  -------------------------------------------------------
//!  |   0   |  0 0 0  | i32 type, value consumes 0 bytes  |
//!  |   0   |  0 0 1  | i32 type, value consumes 1 bytes  |
//!  |   0   |  0 1 0  | i32 type, value consumes 2 bytes  |
//!  |   0   |  0 1 1  | i32 type, value consumes 3 bytes  |
//!  |   0   |  1 0 0  | i32 type, value consumes 4 bytes  |
//!  |-----------------------------------------------------|
//!  |   0   |  1 0 1  | i64 type, value consumes 0 bytes  |
//!  |   0   |  1 1 0  | invalid			   |
//!  |   0   |  1 1 1  | invalid			   |
//!  |-----------------------------------------------------|
//!  |   1   |  0 0 0  | i64 type, value consumes 1 bytes  |
//!  |   1   |  0 0 1  | i64 type, value consumes 2 bytes  |
//!  |   1   |  0 1 0  | i64 type, value consumes 3 bytes  |
//!  |   1   |  0 1 1  | i64 type, value consumes 4 bytes  |
//!  |   1   |  1 0 0  | i64 type, value consumes 5 bytes  |
//!  |   1   |  1 0 1  | i64 type, value consumes 6 bytes  |
//!  |   1   |  1 1 0  | i64 type, value consumes 7 bytes  |
//!  |   1   |  1 1 1  | i64 type, value consumes 8 bytes  |
//!  |_______|_________|___________________________________|
//!  
//! Note:
//! `0 0 0 0` - represents an `i32` number holding zero.
//! `0 1 0 1` - represents an `i64` number holding zero.
//!  

mod parse;

pub use parse::parse_app_tx;
