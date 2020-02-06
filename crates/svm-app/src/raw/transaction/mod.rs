//!     Execute `AppTransaction` Raw Format Version 0.0
//!  ------------------------------------------------------
//!  |   proto     |                                       |
//!  |  version    |          `AppAddress`                 |
//!  |  encoding   |           (20 bytes)                  |
//!  |_____________|_______________________________________|
//!  |                                                     |
//!  |             function index encoding                 |
//!  |_____________________________________________________|
//!  |  func-buf  |                                        |
//!  |  #length   |           func-buf blob                |
//!  |  encoding  |                                        |
//!  |____________|________________________________________|
//!  |   func    |   func    |   func    |                 |
//!  |  arg #1   |  arg #2   |  arg #3   |                 |
//!  |  layout   |  layout   |  layout   |     . . . .     |
//!  | enccoding | encoding  | encoding  |                 |
//!  |___________|___________|___________|_________________|
//!  |   func    |   func    |   func    |                 |
//!  |  arg #1   |  arg #2   |  arg #3   |     . . . .     |                 
//!  |  value    |   value   |   value   |                 |
//!  |___________|___________|___________|_________________|
//!

mod parse;

pub use parse::parse_app_tx;
