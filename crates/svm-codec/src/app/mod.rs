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

mod serialize;
mod wire;

pub use serialize::{DefaultAppDeserializer, DefaultAppSerializer};
pub use wire::{decode_spawn_app, encode_spawn_app};
