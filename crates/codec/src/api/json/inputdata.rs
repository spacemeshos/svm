use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as Json;

use std::convert::TryFrom;

use svm_abi_decoder::CallData;
use svm_abi_encoder::{ByteSize, Encoder};
use svm_sdk_types::value::{Composite, Primitive, Value as SdkValue};
use svm_sdk_types::{Address, Amount};

use super::serde_types::{AddressWrapper, EncodedData, HexBlob};
use super::JsonSerdeUtils;
use crate::api::json::JsonError;

/// Given an `Input Data` JSON, encodes it into a binary `Input Data`
/// and returns the result wrapped with a JSON.
///
/// ```json
/// {
///   "data": "FFC103..."
/// }
/// ```
pub fn encode_inputdata(json: &str) -> Result<Json, JsonError> {
    let decoded = DecodedInputData::new(json)?;
    let calldata = HexBlob(decoded.encode().unwrap());
    Ok(EncodedData { data: calldata }.to_json())
}

pub fn decode_raw_input(data: &[u8]) -> Result<Json, JsonError> {
    let calldata = CallData::new(data);
    Ok(calldata_to_json(calldata))
}

/// Given a binary `Calldata` (wrapped within a JSON), decodes it into a JSON
pub fn decode_inputdata(json: &str) -> Result<Json, JsonError> {
    let encoded = EncodedData::from_json_str(json)?;
    let calldata = CallData::new(&encoded.data.0);
    Ok(calldata_to_json(calldata))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) struct DecodedInputData {
    abi: Vec<TySig>,
    data: Vec<Json>,
}

impl DecodedInputData {
    pub fn new(json: &str) -> Result<Self, JsonError> {
        let decoded = Self::from_json_str(json)?;

        if decoded.abi.len() != decoded.data.len() {
            Err(JsonError::InvalidField {
                path: "data".to_string(),
            })
        } else {
            Ok(decoded)
        }
    }

    /// Like `Self::zip`, but in borrowed form.
    fn zip_ref(&self) -> impl Iterator<Item = (&TySig, &Json)> {
        self.abi.iter().zip(self.data.iter())
    }

    fn zip(self) -> impl Iterator<Item = (TySig, Json)> {
        self.abi.into_iter().zip(self.data.into_iter())
    }

    fn cap(&self) -> Result<usize, JsonError> {
        self.zip_ref()
            .map(|(ty, raw)| ty.value_byte_size(&raw))
            .sum()
    }

    pub fn encode(self) -> Result<Vec<u8>, JsonError> {
        let cap = self.cap()?;
        let mut buf = svm_sdk_std::Vec::with_capacity(cap);

        self.zip()
            .try_for_each(|(ty, raw)| encode_value(ty, raw).map(|value| value.encode(&mut buf)))?;

        Ok(buf.as_slice().to_vec())
    }
}

impl JsonSerdeUtils for DecodedInputData {}

pub(crate) fn calldata_to_json(mut calldata: CallData) -> Json {
    let mut abi = vec![];
    let mut data = vec![];

    while let Some(value) = calldata.next().into() {
        abi.push(sdk_value_utils::ty_sig_of_sdk_value(&value));
        data.push(sdk_value_utils::sdk_value_to_json(value));
    }

    json!({ "abi": abi, "data": data })
}

mod sdk_value_utils {
    use svm_types::{Address, BytesPrimitive};

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
                Primitive::Address(x) => AddressWrapper(Address::new(x.as_slice())).to_json(),
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
                    let ty = &values.last().unwrap();
                    Json::Array(vec![ty_sig_of_sdk_value(ty)])
                }
            }
        }
    }

    pub fn sdk_value_from_json(json: Json, ty_sig: TySigPrim) -> Option<SdkValue> {
        fn json_as_numeric<N>(json: Json) -> Option<SdkValue>
        where
            N: TryFrom<i64> + Into<SdkValue>,
        {
            json.as_i64()
                .and_then(|n| N::try_from(n).ok())
                .map(Into::into)
        }

        match ty_sig {
            TySigPrim::Bool => json.as_bool().map(Into::into),
            TySigPrim::Amount => json
                .as_u64()
                .map(|val| SdkValue::Primitive(Primitive::Amount(Amount(val)))),
            TySigPrim::Address => serde_json::from_value::<AddressWrapper>(json)
                .ok()
                .map(|addr| {
                    let addr = svm_sdk_types::Address::from(*addr.0.as_ref());
                    SdkValue::Primitive(Primitive::Address(addr))
                }),
            TySigPrim::I8 => json_as_numeric::<i8>(json),
            TySigPrim::U8 => json_as_numeric::<u8>(json),
            TySigPrim::I16 => json_as_numeric::<i16>(json),
            TySigPrim::U16 => json_as_numeric::<u16>(json),
            TySigPrim::I32 => json_as_numeric::<i32>(json),
            TySigPrim::U32 => json_as_numeric::<u32>(json),
            TySigPrim::I64 => json_as_numeric::<i64>(json),
            // [`u64`] is the only JSON integer type which doesn't fit into `i64`.
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

                let elems = value.as_array().ok_or(JsonError::InvalidField {
                    path: "calldata".to_string(),
                })?;

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

fn encode_value(ty: TySig, value: Json) -> Result<SdkValue, JsonError> {
    match ty {
        TySig::Array(types) => encode_array(&types, value),
        TySig::Prim(prim) => {
            sdk_value_utils::sdk_value_from_json(value, prim).ok_or(JsonError::InvalidField {
                path: "calldata".to_string(),
            })
        }
    }
}

fn encode_array(types: &[TySig], mut value: Json) -> Result<SdkValue, JsonError> {
    assert_eq!(types.len(), 1);

    let ty = &types[0];

    let mut value = value.take();
    let elems = value.as_array_mut().ok_or(JsonError::InvalidField {
        path: "calldata".to_string(),
    })?;

    let mut vec = svm_sdk_std::Vec::with_capacity(10);

    for elem in elems.iter_mut() {
        let elem = encode_value(ty.clone(), elem.take())?;

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

impl JsonSerdeUtils for CalldataEncoded {}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($abi:expr, $data:expr) => {{
            let json = json!({"abi": $abi, "data": $data });

            let encoded = encode_inputdata(&json.to_string()).unwrap();
            let decoded = decode_inputdata(&encoded.to_string()).unwrap();

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
