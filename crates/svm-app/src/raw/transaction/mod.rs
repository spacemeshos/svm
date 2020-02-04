//!    Execute `AppTransaction` Raw Format Version 0.0
//!  -------------------------------------------------------
//!  |   proto    |                                        |
//!  |  version   |             `AppAddress`               |
//!  |  (2 bytes) |              (20 bytes)                |
//!  |____________|________________________________________|
//!  |                                                     |
//!  |        func index (2 bytes, Big-Endian)             |
//!  |_____________________________________________________|
//!  |  func-buf  |                                        |
//!  |  #length   |           func-buf blob                |
//!  | (2 bytes)  |                                        |
//!  |____________|________________________________________|
//!  |   func    |   func    |   func    |                 |
//!  |  arg #1   |  arg #2   |  arg #3   |                 |
//!  |   value   |   value   |   value   |     . . . .     |
//!  | (i32/i64) | (i32/i64) | (i32/i64) |                 |
//!  |___________|___________|___________|_________________|
//!

mod parse;

pub use parse::parse_app_tx;
