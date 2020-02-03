//!    Execute `AppTransaction` Raw Format Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   proto    |                                        |
//!  |  version   |             `AppAddress`               |
//!  |  (4 bytes) |              (20 bytes)                |
//!  |____________|________________________________________|
//!  |             |                                       |
//!  |  func name  |                                       |
//!  |   length    |          func name (UTF-8)            |
//!  |  (1 byte)   |                                       |
//!  |_____________|_______________________________________|
//!  |  func-buf  |  func-buf  |  func-buf  |              |
//!  |   #slices  |   slice #1 |  slice #1  |              |
//!  |            |    len     |            |   . . . .    |
//!  | (1 byte)   | (2 bytes)  |   value    |              |
//!  |____________|____________|____________|______________|
//!  | func-args |    func   |    func   |                 |
//!  |   #args   |   arg #1  |   arg #1  |                 |
//!  |           |    type   |   value   |     . . . .     |
//!  | (1 byte)  | (1 byte)  | (u32/u64) |                 |
//!  |___________|___________|___________|_________________|
//!

mod parse;

pub use parse::parse_app_tx;
