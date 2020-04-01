mod decoder;
mod encoder;
mod layout;

pub use decoder::decode_wasm_value;
pub use encoder::encode_wasm_value;
pub use layout::{wasm_value_layout, WasmValueLayout};
