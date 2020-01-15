//!    Execute `AppTransaction` Raw Format Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   proto    |                                        |
//!  |  version   |           `TemplateAddress`            |
//!  |  (4 bytes) |             (20 bytes)                 |
//!  |____________|________________________________________|
//!  |             |                                       |
//!  |  func name  |                                       |
//!  |   length    |          func name (UTF-8)            |
//!  |  (1 byte)   |                                       |
//!  |_____________|_______________________________________|
//!  |  func-buf  |  func-buf  |  func-buf  |              |
//!  |  #params   |  param #1  |   param    |              |
//!  |            |   len      |     #1     |    . . .     |
//!  | (1 byte)   | (2 bytes)  |   value    |              |
//!  |____________|____________|____________|______________|
//!  |   func    |   func    |    func   |                 |
//!  |  #params  |  param #1 |  param #1 |                 |
//!  |           |    type   |   value   |      . . . .    |
//!  | (1 byte)  | (1 byte)  | (i32/i64) |                 |
//!  |_________ _|___________|___________|_________________|
//!

mod parse;

pub use parse::parse_app_tx;
