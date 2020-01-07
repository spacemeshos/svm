//!         Spawn `App` Wire Protocol Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   proto    |                                        |
//!  |  version   |       `AppTemplate` `Address`          |
//!  |  (4 bytes) |            (20 bytes)                  |
//!  |____________|________________________________________|
//!  |                                                     |
//!  |                  Creator `Address`                  |
//!  |                     (20 bytes)                      |
//!  |_____________________________________________________|
//!

mod parse;
mod serialize;

pub use parse::parse_app;
pub use serialize::{AppJsonDeserializer, AppJsonSerializer};
