//! `AppTemplate` Raw Format
//!
//!  +_____________________________________________________+
//!  |            |                                        |
//!  |  version   |               name                     |
//!  |  (2 bytes) |             (String)                   |
//!  +____________|________________________________________+
//!  |               |                                     |
//!  |  Code #bytes  |          Code (WASM)                |
//!  |   (4 bytes)   |                                     |
//!  +_______________|_____________________________________+
//!  |               |             |         |             |
//!  |  Data-Layout  |  var #0     |         |   var #N    |
//!  |  #variables   |  length     |  . . .  |   length    |
//!  +_______________|_____________|_________|_____________+
//!
//!

mod raw;
mod serialize;
mod validate;

pub use raw::{decode_deploy_template, encode_deploy_template};
pub use serialize::{DefaultAppTemplateDeserializer, DefaultAppTemplateSerializer};
pub use validate::validate_template;
