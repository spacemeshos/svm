//!                   `Spawn-App` Raw Format
//!  ----------------------------------------------------------
//!  |               |                                        |
//!  |    `version`  |        `AppTemplate` (`Address`)       |
//!  |_______________|________________________________________|
//!  |            |                                           |
//!  | ctor index |            ctor-buf (`func-buf`)          |
//!  |____________|___________________________________________|
//!  |                                                        |
//!  |                 ctor-args (`func-args``)               |
//!  |________________________________________________________|
//!
//!

mod parse;
mod serialize;

pub use parse::parse_app;
pub use serialize::{DefaultAppDeserializer, DefaultAppSerializer};
