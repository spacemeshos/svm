//!       `AppTemplate` Raw Format Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   format   |                |                       |
//!  |  version   |  name length   |     name (UTF-8)      |
//!  |  (4 bytes) |   (1 byte)     |                       |
//!  |____________|________________|_______________________|
//!  |                                                     |
//!  |                 author `Address`                    |
//!  |                  (20 bytes)                         |
//!  |_____________________________________________________|
//!  |             |                                       |
//!  |   #admins   |         admins `Address`-(es)         |
//!  |  (2 bytes)  |          (20 bytes each)              |
//!  |_____________|_______________________________________|
//!  |             |                                       |
//!  |   #deps     |           dependencies                |
//!  |  (2 bytes)  |              (TBD)                    |
//!  |_____________|_______________________________________|
//!  |             |                                       |
//!  |   #pages    |            code length                |
//!  |  (2 bytes)  |             (8 bytes)                 |
//!  |_____________|_______________________________________|
//!  |                                                     |
//!  |             app-template code (wasm)                |
//!  |_____________________________________________________|
//!

mod parse;
mod serialize;
mod validate;

pub use parse::parse_template;
pub use serialize::{AppTemplateJsonDeserializer, AppTemplateJsonSerializer};
pub use validate::validate_template;
