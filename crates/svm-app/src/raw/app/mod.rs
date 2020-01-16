//!          `Spaw-App` Raw Format Version 0.0.0.0
//!  ----------------------------------------------------------
//!  |     proto     |                                        |
//!  |    version    |        `AppTemplate` `Address`         |
//!  |    (4 bytes)  |            (20 bytes)                  |
//!  |_______________|________________________________________|
//!  |  ctor-buf  |  ctor-buf  |  ctor-buf  |                 |
//!  |    #args   |  arg #1    |   arg #1   |                 |
//!  |            |   len      |            |     . . . .     |
//!  | (1 byte)   | (2 bytes)  |   value    |                 |
//!  |____________|____________|____________|_________________|
//!  | ctor-func | ctor-func | ctor-func |                    |
//!  |  #args    |  arg #1   |   arg #1  |                    |
//!  |           |   type    |   value   |      . . . .       |
//!  | (1 byte)  | (1 byte)  | (i32/i64) |                    |
//!  |___________|___________|___________|____________________|
//!

mod parse;
mod serialize;

pub use parse::parse_app;
pub use serialize::{AppJsonDeserializer, AppJsonSerializer};
