//              Spawn-App Raw Format
//
//  +-------------------------------------------------------+
//  |             |                                         |
//  |  `version`  |        `Template` (`Address`)           |
//  |_____________|_________________________________________|
//  |               |                                       |
//  | ctor (String) |           ctor `CallData`             |
//  +_______________|_______________________________________+
//

mod raw;
mod serialize;

pub use raw::{decode_spawn_app, encode_spawn_app};
pub use serialize::{DefaultAppDeserializer, DefaultAppSerializer};
