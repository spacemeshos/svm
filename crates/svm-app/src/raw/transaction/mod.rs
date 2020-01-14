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
//!  |           |              |         |                |
//!  |  #args    |  arg #1 type |  arg #1 |    . . . .     |
//!  | (1 byte)  |  (1 byte)    |  value  |                |
//!  |___________|______________|_________|________________|
//!

mod parse;

pub use parse::parse_app_tx;
