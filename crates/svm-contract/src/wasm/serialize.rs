use crate::traits::{ContractDeserializer, ContractSerializer};
use crate::wasm::Contract;

pub struct WasmContractJsonSerializer;
pub struct WasmContractJsonDeserializer;

impl ContractSerializer for WasmContractJsonSerializer {
    fn serialize(contract: &Contract) -> Vec<u8> {
        let s = serde_json::to_string(&contract).unwrap();
        s.into_bytes()
    }
}

impl ContractDeserializer for WasmContractJsonDeserializer {
    fn deserialize(bytes: Vec<u8>) -> Contract {
        let s = unsafe { String::from_utf8_unchecked(bytes) };

        serde_json::from_str(s.as_str()).unwrap()
    }
}
