//!      Deploy Contract Wire Protocol Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   proto    |                |                       |
//!  |  version   |  name length   |     name (UTF-8)      |
//!  |  (4 bytes) |   (1 byte)     |                       |
//!  |____________|________________|_______________________|
//!  |                                                     |
//!  |                      author                         |
//!  |                    (32 bytes)                       |
//!  |_____________________________________________________|
//!  |             |                                       |
//!  |  #admins    |         admins addresses              |
//!  |  (2 bytes)  |          (32 bytes each)              |
//!  |_____________|_______________________________________|
//!  |           |                                         |
//!  |   #deps   |           dependencies                  |
//!  | (2 bytes) |              (TBD)                      |
//!  |___________|_________________________________________|
//!  |                |                                    |
//!  |  code length   |              code                  |
//!  |   (8 bytes)    |             (wasm)                 |
//!  |________________|____________________________________|
//!
//!
//!
//!      Exec Contract Wire Protocol Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   proto    |                                        |
//!  |  version   |          contract address              |
//!  |  (4 bytes) |             (32 bytes)                 |
//!  |____________|________________________________________|
//!  |                                                     |
//!  |                  sender address                     |
//!  |                    (32 bytes)                       |
//!  |_____________________________________________________|
//!  |             |                                       |
//!  |  func name  |                                       |
//!  |   length    |          func name (UTF-8)            |
//!  |  (1 byte)   |                                       |
//!  |_____________|_______________________________________|
//!  |           |              |         |                |
//!  |  #args    |  arg #1 type |  arg #1 |    . . . .     |
//!  | (1 byte)  |  (1 byte)    |   data  |                |
//!  |___________|______________|_________|________________|
//!

mod build;
mod error;
mod field;
mod parse;
mod validate;

pub use crate::wasm::WasmContract;
pub use build::WireContractBuilder;
pub use error::ContractError;

pub use parse::parse_contract;
pub use validate::validate_contract;
