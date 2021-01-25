//! `AppTemplate` Raw Format
//!  ______________________________________________________
//!  |            |                |                       |
//!  |  version   |  name length   |         name          |
//!  |            |  (varuint14)   |        (UTF-8)        |
//!  +____________|________________|_______________________+
//!  |               |                                     |
//!  |  Code #bytes  |          Code (WASM)                |
//!  |   (4 bytes)   |                                     |
//!  +_______________|_____________________________________+
//!  |               |             |         |             |
//!  |  Data-Layout  |  var #0     |         |   var #N    |
//!  |  #variables   |  length     |  . . .  |   length    |
//!  |  (varuint14)  | (varuint14) |         | (varuint14) |
//!  +_______________|_____________|_________|_____________+
//!
//!

mod serialize;
mod validate;
mod wire;

pub use serialize::{DefaultAppTemplateDeserializer, DefaultAppTemplateSerializer};
pub use validate::validate_template;
pub use wire::{decode_deploy_template, encode_deploy_template};
