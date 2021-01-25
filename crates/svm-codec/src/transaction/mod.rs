//! Execute `AppTransaction` Raw Format Version 0.0
//!
//!  +--------------------------------------------+
//!  |             |                              |
//!  |  version    |          `AppAddress`        |
//!  |             |           (Address)          |
//!  |_____________|______________________________|
//!  |                                            |
//!  |            Function (String)               |
//!  |____________________________________________|
//!  |              |                             |
//!  |  `Calldata`  |       `Calldata`            |
//!  |   #length    |         (blob)              |
//!  |____________  |_____________________________|
//!
//!

mod wire;

pub use wire::{decode_exec_app, encode_exec_app};
