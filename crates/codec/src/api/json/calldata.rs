use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as Json;

use svm_abi_decoder::CallData;
use svm_abi_encoder::{ByteSize, Encoder};
use svm_sdk_types::value::{Composite, Primitive, Value as SdkValue};
use svm_sdk_types::{Address, Amount};

use super::wrappers::AddressWrapper;
use super::wrappers::HexBlob;
use super::BetterConversionToJson;
use crate::api::json::{self, JsonError};

/// Given a `Calldata` JSON, encodes it into a binary `Calldata`
/// and returns the result wrapped with a JSON
pub fn encode_calldata(json: Json) -> Result<Json, JsonError> {
    let decoded = DecodedCallData::new(json)?;
    let encoded = EncodedCallData::from(decoded);
    Ok(serde_json::to_value(encoded).unwrap())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct DecodedCallData {
    abi: Vec<TySig>,
    data: Vec<Json>,
}

impl DecodedCallData {
    fn new(json: Json) -> Result<Self, JsonError> {
        let wrapper: Self =
            serde_json::from_value(json.clone()).map_err(JsonError::from_serde::<Self>)?;

        if wrapper.abi.len() != wrapper.data.len() {
            Err(JsonError::InvalidField {
                field: "data".to_string(),
                reason: "`abi` and `data` must be of the same length".to_string(),
            })
        } else {
            Ok(wrapper)
        }
    }

    fn zip(&self) -> impl Iterator<Item = (&TySig, &Json)> {
        self.abi.iter().zip(self.data.iter())
    }

    fn cap(&self) -> Result<usize, JsonError> {
        self.zip().map(|(ty, raw)| ty.value_byte_size(&raw)).sum()
    }

    fn encode(&self) -> Result<Vec<u8>, JsonError> {
        let cap = self.cap()?;
        let mut buf = svm_sdk_std::Vec::with_capacity(cap);

        self.zip()
            .try_for_each(|(ty, raw)| encode_value(ty, &raw).map(|value| value.encode(&mut buf)))?;

        Ok(buf.as_slice().to_vec())
    }
}

impl BetterConversionToJson for DecodedCallData {
    fn type_of_field_as_str(_field: &str) -> Option<&str> {
        Some("array")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EncodedCallData {
    calldata: HexBlob<Vec<u8>>,
}

impl EncodedCallData {
    fn new(json: Json) -> Result<Self, JsonError> {
        serde_json::from_value(json).map_err(|e| JsonError::from_serde::<Self>(e))
    }

    fn decode(self) -> CallData {
        CallData::new(&self.calldata.0)
    }
}

impl From<DecodedCallData> for EncodedCallData {
    fn from(decoded: DecodedCallData) -> Self {
        let calldata = HexBlob(decoded.encode().unwrap());
        Self { calldata }
    }
}

impl BetterConversionToJson for EncodedCallData {
    fn type_of_field_as_str(field: &str) -> Option<&str> {
        match field {
            "calldata" => Some("string"),
            _ => None,
        }
    }
}

fn calldata_to_json(mut calldata: CallData) -> Json {
    let mut abi = vec![];
    let mut data = vec![];

    while let Some(value) = calldata.next().into() {
        abi.push(sdk_value_utils::ty_sig_of_sdk_value(&value));
        data.push(sdk_value_utils::sdk_value_to_json(value));
    }

    json!({ "abi": abi, "data": data })
}

/// Given a binary `Calldata` (wrapped within a JSON), decodes it into a JSON
pub fn decode_calldata(json: Json) -> Result<Json, JsonError> {
    let encoded_calldata = EncodedCallData::new(json)?;
    let calldata = encoded_calldata.decode();
    Ok(calldata_to_json(calldata))
}

mod sdk_value_utils {
    use super::*;

    /// Given a [`svm_sdk_types::value::Value`], encodes its value as a
    /// JSON value. This function, together with [`ty_sig_of_sdk_value`], can
    /// give a **ful** overview over some values, with both its type signature
    /// and its value.
    pub fn sdk_value_to_json(value: SdkValue) -> Json {
        match value {
            SdkValue::Primitive(prim) => match prim {
                Primitive::Bool(x) => json!(x),
                Primitive::I8(x) => json!(x),
                Primitive::U8(x) => json!(x),
                Primitive::I16(x) => json!(x),
                Primitive::U16(x) => json!(x),
                Primitive::I32(x) => json!(x),
                Primitive::U32(x) => json!(x),
                Primitive::I64(x) => json!(x),
                Primitive::U64(x) => json!(x),
                Primitive::Amount(x) => json!(x.0),
                Primitive::Address(x) => {
                    let hex_blob = HexBlob(x.as_slice());
                    serde_json::to_value(hex_blob).unwrap()
                }
                _ => unreachable!(),
            },
            SdkValue::Composite(Composite::Vec(values)) => Json::Array(
                values
                    .into_iter()
                    .map(|sdk_value| sdk_value_to_json(sdk_value))
                    .collect(),
            ),
        }
    }

    /// Given a [`svm_sdk_types::value::Value`], encodes its type signature as a
    /// JSON value.
    pub fn ty_sig_of_sdk_value(value: &SdkValue) -> Json {
        match value {
            SdkValue::Primitive(prim) => match prim {
                Primitive::Bool(_) => "bool",
                Primitive::I8(_) => "i8",
                Primitive::U8(_) => "u8",
                Primitive::I16(_) => "i16",
                Primitive::U16(_) => "u16",
                Primitive::I32(_) => "i32",
                Primitive::U32(_) => "u32",
                Primitive::I64(_) => "i64",
                Primitive::U64(_) => "u64",
                Primitive::Amount(_) => "amount",
                Primitive::Address(_) => "address",
                _ => unreachable!(),
            }
            .into(),
            SdkValue::Composite(Composite::Vec(values)) => {
                if values.is_empty() {
                    Json::Null
                } else {
                    Json::Array(
                        values
                            .iter()
                            .map(|sdk_value| ty_sig_of_sdk_value(sdk_value))
                            .collect(),
                    )
                }
            }
        }
    }

    pub fn sdk_value_from_json(json: &Json, ty_sig: TySigPrim) -> Option<SdkValue> {
        match ty_sig {
            TySigPrim::Bool => json.as_bool().map(Into::into),
            TySigPrim::Amount => json
                .as_u64()
                .map(|val| SdkValue::Primitive(Primitive::Amount(Amount(val)))),
            TySigPrim::Address => serde_json::from_value::<AddressWrapper>(json.clone())
                .ok()
                .map(|addr| {
                    SdkValue::Primitive(Primitive::Address(Address::from(addr.0.as_ptr())))
                }),
            // FIXME: boundaries
            TySigPrim::I8 => json.as_i64().map(|n| (n as i8).into()),
            TySigPrim::U8 => json.as_i64().map(|n| (n as u8).into()),
            TySigPrim::I16 => json.as_i64().map(|n| (n as i16).into()),
            TySigPrim::U16 => json.as_i64().map(|n| (n as u16).into()),
            TySigPrim::I32 => json.as_i64().map(|n| (n as i32).into()),
            TySigPrim::U32 => json.as_i64().map(|n| (n as u32).into()),
            TySigPrim::I64 => json.as_i64().map(Into::into),
            TySigPrim::U64 => json.as_u64().map(Into::into),
        }
    }
}

// See <https://serde.rs/enum-representations.html>.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum TySig {
    Prim(TySigPrim),
    Array(Vec<TySig>),
}

impl TySig {
    fn value_byte_size(&self, value: &Json) -> Result<usize, JsonError> {
        let byte_size = match self {
            TySig::Array(types) => {
                assert_eq!(types.len(), 1);

                let ty = &types[0];

                // we initialize `byte_size` for the `length` marker.
                let mut byte_size = 1;

                let json = json!({ "calldata": value });
                let elems = json::as_array(&json, "calldata")?;

                for elem in elems {
                    byte_size += ty.value_byte_size(elem)?;
                }

                byte_size
            }
            TySig::Prim(prim) => match prim {
                TySigPrim::Bool => bool::max_byte_size(),
                TySigPrim::I8 => i8::max_byte_size(),
                TySigPrim::U8 => u8::max_byte_size(),
                TySigPrim::I16 => i16::max_byte_size(),
                TySigPrim::U16 => u16::max_byte_size(),
                TySigPrim::I32 => i32::max_byte_size(),
                TySigPrim::U32 => u32::max_byte_size(),
                TySigPrim::I64 => i64::max_byte_size(),
                TySigPrim::U64 => u64::max_byte_size(),
                TySigPrim::Amount => Amount::max_byte_size(),
                TySigPrim::Address => Address::max_byte_size(),
            },
        };
        Ok(byte_size)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TySigPrim {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Amount,
    Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum TyPrimSdkValue {
    Bool(bool),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Amount(u64),
    Address(AddressWrapper),
}

fn encode_value(ty: &TySig, value: &Json) -> Result<SdkValue, JsonError> {
    match ty {
        TySig::Array(types) => encode_array(types, value),
        TySig::Prim(prim) => {
            sdk_value_utils::sdk_value_from_json(value, *prim).ok_or(JsonError::InvalidField {
                field: "calldata".to_string(),
                reason: "not according to type".to_string(),
            })
        }
    }
}

fn encode_array(types: &[TySig], value: &Json) -> Result<SdkValue, JsonError> {
    assert_eq!(types.len(), 1);

    let ty = &types[0];

    let json = json!({ "calldata": value });
    let elems = json::as_array(&json, "calldata")?;

    let mut vec = svm_sdk_std::Vec::with_capacity(10);

    for elem in elems {
        let elem = encode_value(ty, elem)?;

        vec.push(elem);
    }

    let c = Composite::Vec(vec);

    Ok(SdkValue::Composite(c))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct CalldataEncoded {
    calldata: HexBlob<Vec<u8>>,
}

//impl CalldataEncoded {
//    fn new(json: &Json) -> Result<Self, JsonError> {
//        serde_json::from_value(json.clone()).map_err(|e| JsonError::from_serde::<Self>(e))
//    }
//}

impl BetterConversionToJson for CalldataEncoded {
    fn type_of_field_as_str(_field: &str) -> Option<&str> {
        Some("string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($abi:expr, $data:expr) => {{
            let json = json!({"abi": $abi, "data": $data });

            let encoded = encode_calldata(json.clone()).unwrap();
            let decoded = decode_calldata(encoded).unwrap();

            assert_eq!(decoded, json);
        }}
    }

    #[test]
    fn encode_calldata_bool() {
        test!(["bool", "bool"], [true, false]);
    }

    #[test]
    fn encode_calldata_i8_u8() {
        test!(["i8", "u8"], [std::i8::MIN as isize, std::u8::MAX as isize]);
    }

    #[test]
    fn encode_calldata_i16_u16() {
        test!(
            ["i16", "u16"],
            [std::i16::MIN as isize, std::u16::MAX as isize]
        );
    }

    #[test]
    fn encode_calldata_i32_u32() {
        test!(
            ["i32", "u32"],
            [std::i32::MIN as isize, std::u32::MAX as isize]
        );
    }

    #[test]
    fn encode_calldata_i64_u64() {
        test!(["i64"], [std::i64::MIN as isize]);
        test!(["u64"], [std::u64::MAX as usize]);
    }

    #[test]
    fn encode_calldata_amount() {
        test!(["amount", "amount"], [10 as u64, 20 as u64]);
    }

    #[test]
    fn encode_calldata_address() {
        let addr = "1020304050607080900010203040506070809000";

        test!(["address"], [addr]);
    }

    #[test]
    fn encode_calldata_array() {
        test!([["u32"]], [[10, 20, 30]]);
        test!([["i8"]], [[-10, 0, 30]]);
        test!([["u32"], ["i8"]], [[10, 20, 30], [-10, 0, 20]]);
    }
}
