//!         `Spaw-App` Raw Format Version 0.0.0.0
//!  ----------------------------------------------------------
//!  |     proto     |                                        |
//!  |    version    |         `AppTemplate` `Address`        |
//!  |    (4 bytes)  |            (20 bytes)                  |
//!  |_______________|________________________________________|
//!  |           |           |         |                      |
//!  |  #inits   |  init #1  | init #1 |       . . . .        |
//!  | (1 byte)  | (1 byte)  |  value  |                      |
//!  |___________|___________|_________|______________________|
//!

mod parse;
mod serialize;

pub use parse::parse_app;
pub use serialize::{AppJsonDeserializer, AppJsonSerializer};
