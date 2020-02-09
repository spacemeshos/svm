//!       `AppTemplate` Raw Format Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   proto    |                |                       |
//!  |  version   |  name length   |         name          |
//!  |  encoding  |                |       (UTF-8)         |
//!  |    (a)     |   (1 byte)     |                       |
//!  |____________|________________|_______________________|
//!  |             |                                       |
//!  |  #app-pages |               code                    |
//!  |  (2 bytes)  |             (8 bytest)                 |
//!  |_____________|_______________________________________|
//!  |                                                     |
//!  |           `AppTemplate` code (wasm)                 |
//!  |_____________________________________________________|
//!
//!
//!
//! (a) Proto Version Encoding
//! ===========================
//!     MSB     non-MSB           Meaning
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

pub use parse::parse_template;
pub use serialize::{AppTemplateJsonDeserializer, AppTemplateJsonSerializer};
pub use validate::validate_template;
