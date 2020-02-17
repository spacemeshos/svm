//!                   `Spawn-App` Raw Format
//!  ----------------------------------------------------------
//!  |               |                                        |
//!  |    `version`  |        `AppTemplate` (`Address`)       |
//!  |_______________|________________________________________|
//!  |                                                        |
//!  |                 ctor-buf (`func-buf`)                  |
//!  |________________________________________________________|
//!  |                                                        |
//!  |                 ctor-args (`func-args``)               |
//!  |________________________________________________________|
//!
//!

mod parse;
mod serialize;

pub use parse::parse_app;
pub use serialize::{AppJsonDeserializer, AppJsonSerializer};
