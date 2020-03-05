//!                  `AppTemplate` Raw Format
//!  -------------------------------------------------------
//!  |            |                |                       |
//!  |  version   |  name length   |         name          |
//!  |    (a)     |   (varuint14)  |        (UTF-8)        |
//!  |____________|________________|_______________________|
//!  |               |                                     |
//!  |  #app-pages   |        `AppTemplate` (wasm)         |
//!  | (`varuint14`) |           (8 bytest)                |
//!  |_______________|_____________________________________|
//!
//!
//!
//! (a) Proto Version Encoding
//! ===========================
//!
//!   ___________________________________________________
//!  |  MSB  |  non-MSB  |         Meaning              |
//!  ---------------------------------------------------
//!  |   1   |  x  x  x  |  Next nibble is relevant too |
//!  |   0   |  x  x  x  |  Next nibble isn't relevant  |
//!  |--------------------------------------------------|
//!
//!  The protocol `#bits` will be a multiplication of 3,
//!  and is encoded in Little-endian as an unsigned-integer.
//!

mod parse;
mod serialize;
mod validate;
mod wire;

pub use parse::parse_template;
pub use serialize::{DefaultAppTemplateDeserializer, DefaultAppTemplateSerializer};
pub use validate::validate_template;
pub use wire::encode_deploy_template;
