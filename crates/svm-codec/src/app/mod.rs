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

mod serialize;
mod wire;

pub use serialize::{DefaultAppDeserializer, DefaultAppSerializer};
pub use wire::{decode_spawn_app, encode_spawn_app};
