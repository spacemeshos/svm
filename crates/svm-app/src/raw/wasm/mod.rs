mod decoder;
mod encoder;
mod layout;

pub use decoder::decode_wasm_value;
pub use encoder::encode_wasm_value;
pub use layout::{
    wasm_value_layout, WasmValueLayout, DO_SKIP, I32_0B, I32_1B, I32_2B, I32_3B, I32_4B, I64_0B,
    I64_1B, I64_2B, I64_3B, I64_4B, I64_5B, I64_6B, I64_7B, I64_8B, NO_MORE,
};
