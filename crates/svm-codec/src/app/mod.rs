//!              `Spawn-App` Raw Format
//!
//!  ----------------------------------------------------------
//!  |             |                                          |
//!  |  `version`  |        `AppTemplate` (`Address`)         |
//!  |_____________|__________________________________________|
//!  |               |                                        |
//!  | ctor (String) |           ctor `CallData`              |
//!  |_______________|________________________________________|
//!

mod raw;
mod serialize;

pub use raw::{decode_spawn_app, encode_spawn_app};
pub use serialize::{DefaultAppDeserializer, DefaultAppSerializer};
