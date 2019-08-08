use crate::types::Tag;

use svm_common::Address;

///        Deploy Contract Wire Protocol Version
///  -------------------------------------------------------
///  |   proto    |                |                       |
///  |  version   |  name length   |     name (UTF-8)      |
///  |  (4 bytes) |   (1 byte)     |                       |
///  |____________|________________|_______________________|
///  |             |                                       |
///  |     tag     |              author                   |
///  |  (4 bytes)  |             (32 bytes)                |
///  |_____________|_______________________________________|
///  |             |                                       |
///  |  #admins    |         admins addresses              |
///  |  (2 bytes)  |          (32 bytes each)              |
///  |_____________|_______________________________________|
///  |           |                                         |
///  |   #deps   |           dependencies                  |
///  | (2 bytes) |              (TBD)                      |
///  |___________|_________________________________________|
///  |                |                                    |
///  |  code length   |              code                  |
///  |   (8 bytes)    |             (wasm)                 |
///  |________________|____________________________________|

#[allow(dead_code)]
pub struct WireContract {
    pub(crate) name: String,
    pub(crate) wasm: Vec<u8>,
    pub(crate) tag: Tag,
    pub(crate) author: Address,
    pub(crate) admins: Vec<Address>,
}
