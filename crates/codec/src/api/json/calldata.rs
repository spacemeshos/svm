use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as Json;

use svm_abi_decoder::CallData;
use svm_abi_encoder::{ByteSize, Encoder};
use svm_sdk_types::value::{Composite, Primitive, Value};
use svm_sdk_types::{Address, Amount};

use super::TypeInformation;
use crate::api::json::{self, JsonError};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct CalldataJsonlike {
    abi: Vec<Ty>,
    data: Vec<Json>,
}

impl TypeInformation for CalldataJsonlike {
    fn type_of_field_as_str(_field: &str) -> Option<&str> {
        Some("array")
    }
}

impl CalldataJsonlike {
    fn new(json: &Json) -> Result<Self, JsonError> {
        let jsonlike: Self =
            serde_json::from_value(json.clone()).map_err(|e| JsonError::from_serde::<Self>(e))?;

        if jsonlike.abi.len() != jsonlike.data.len() {
            Err(JsonError::InvalidField {
                field: "data".to_string(),
                reason: "`abi` and `data` must be of the same length".to_string(),
            })
        } else {
            Ok(jsonlike)
        }
    }

    fn zip(&self) -> impl Iterator<Item = (&Ty, &Json)> {
        self.abi.iter().zip(self.data.iter())
    }

    fn cap(&self) -> Result<usize, JsonError> {
        self.zip().map(|(ty, raw)| ty.value_byte_size(&raw)).sum()
    }

    fn encode_to_buf(&self) -> Result<svm_sdk_std::Vec<u8>, JsonError> {
        let cap = self.cap()?;
        let mut buf = svm_sdk_std::Vec::with_capacity(cap);

        self.zip()
            .try_for_each(|(ty, raw)| encode_value(ty, &raw).map(|value| value.encode(&mut buf)))?;

        Ok(buf)
    }
}

/// Given a `Calldata` JSON, encodes it into a binary `Calldata`
/// and returns the result wrapped with a JSON
pub fn encode_calldata(json: &Json) -> Result<Json, JsonError> {
    let jsonlike = CalldataJsonlike::new(json)?;
    let buf = jsonlike.encode_to_buf()?;
    let calldata = json::bytes_to_str(&buf);
    let json = json!({ "calldata": calldata });

    Ok(json)
}

/// Given a binary `Calldata` (wrapped within a JSON), decodes it into a JSON
pub fn decode_calldata(json: &Json) -> Result<Json, JsonError> {
    let data = json::as_string(json, "calldata")?;
    let calldata = json::str_to_bytes(&data, "calldata")?;
    let mut calldata = CallData::new(&calldata);

    let mut abi = Vec::<Json>::new();
    let mut data = Vec::<Json>::new();

    while let Some(value) = calldata.next().into() {
        let (ty, item) = value_as_json(&value);

        abi.push(ty);
        data.push(item);
    }

    let result = json!({ "abi": abi, "data": data });

    Ok(result)
}

fn value_as_json(value: &Value) -> (Json, Json) {
    match value {
        Value::Primitive(p) => primitive_as_json(p),
        Value::Composite(c) => composite_as_json(c),
    }
}

fn primitive_as_json(p: &Primitive) -> (Json, Json) {
    match p {
        Primitive::Bool(b) => (Json::String("bool".into()), json!(b)),
        Primitive::Amount(a) => (Json::String("amount".into()), json!(a.0)),
        Primitive::I8(n) => (Json::String("i8".into()), json!(n)),
        Primitive::U8(n) => (Json::String("u8".into()), json!(n)),
        Primitive::I16(n) => (Json::String("i16".into()), json!(n)),
        Primitive::U16(n) => (Json::String("u16".into()), json!(n)),
        Primitive::I32(n) => (Json::String("i32".into()), json!(n)),
        Primitive::U32(n) => (Json::String("u32".into()), json!(n)),
        Primitive::I64(n) => (Json::String("i64".into()), json!(n)),
        Primitive::U64(n) => (Json::String("u64".into()), json!(n)),
        Primitive::Address(addr) => {
            let s = json::bytes_to_str(addr.as_slice());
            (Json::String("address".into()), json!(s))
        }
        Primitive::None => unreachable!(),
        Primitive::Unit => unreachable!(),
    }
}

fn composite_as_json(c: &Composite) -> (Json, Json) {
    let slice: &[Value] = match c {
        Composite::Vec(inner) => inner.as_slice(),
    };

    if slice.is_empty() {
        return (Json::Null, Json::Array(std::vec::Vec::new()));
    }

    let mut types: Vec<Json> = Vec::new();
    let mut values: Vec<Json> = Vec::new();

    for elem in slice {
        let (ty, value) = value_as_json(elem);

        types.push(ty);
        values.push(value);
    }

    // TODO: assert that all `types` are the same
    let ty = types.pop().unwrap();

    (Json::Array(vec![ty]), Json::Array(values))
}

// See <https://serde.rs/enum-representations.html>.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Ty {
    Prim(TyPrim),
    Array(Vec<Ty>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TyPrim {
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

impl Ty {
    fn value_byte_size(&self, value: &Json) -> Result<usize, JsonError> {
        let byte_size = match self {
            Ty::Array(types) => {
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
            Ty::Prim(prim) => match prim {
                TyPrim::Bool => bool::max_byte_size(),
                TyPrim::I8 => i8::max_byte_size(),
                TyPrim::U8 => u8::max_byte_size(),
                TyPrim::I16 => i16::max_byte_size(),
                TyPrim::U16 => u16::max_byte_size(),
                TyPrim::I32 => i32::max_byte_size(),
                TyPrim::U32 => u32::max_byte_size(),
                TyPrim::I64 => i64::max_byte_size(),
                TyPrim::U64 => u64::max_byte_size(),
                TyPrim::Amount => Amount::max_byte_size(),
                TyPrim::Address => Address::max_byte_size(),
            },
        };
        //      return Err(JsonError::InvalidField {
        //          field: "abi".to_string(),
        //          reason: format!("invalid ABI type: `{}`", ty),
        //      })

        Ok(byte_size)
    }
}

impl ToString for Ty {
    fn to_string(&self) -> String {
        match self {
            Self::Array(_) => "array".to_string(),
            Self::Prim(prim) => match prim {
                TyPrim::Bool => "bool".to_string(),
                TyPrim::I8 => "i8".to_string(),
                TyPrim::U8 => "u8".to_string(),
                TyPrim::I16 => "i16".to_string(),
                TyPrim::U16 => "u16".to_string(),
                TyPrim::I32 => "i32".to_string(),
                TyPrim::U32 => "u32".to_string(),
                TyPrim::I64 => "i64".to_string(),
                TyPrim::U64 => "u64".to_string(),
                TyPrim::Amount => "amount".to_string(),
                TyPrim::Address => "address".to_string(),
            },
        }
    }
}

fn encode_value(ty: &Ty, value: &Json) -> Result<Value, JsonError> {
    match ty {
        Ty::Array(types) => encode_array(types, value),
        Ty::Prim(prim) => {
            let json = json!({ "calldata": value });

            macro_rules! encode {
                ($func:ident) => {{
                    json::$func(&json, "calldata")?.into()
                }};
            }

            let value: Value = match prim {
                TyPrim::Bool => encode!(as_bool),
                TyPrim::I8 => encode!(as_i8),
                TyPrim::U8 => encode!(as_u8),
                TyPrim::I16 => encode!(as_i16),
                TyPrim::U16 => encode!(as_u16),
                TyPrim::I32 => encode!(as_i32),
                TyPrim::U32 => encode!(as_u32),
                TyPrim::I64 => encode!(as_i64),
                TyPrim::U64 => encode!(as_u64),
                TyPrim::Amount => encode!(as_amount),
                TyPrim::Address => {
                    let addr: svm_types::Address = json::as_addr(&json, "calldata")?;

                    let bytes = addr.bytes();
                    let addr: Address = bytes.into();

                    addr.into()
                }
            };

            Ok(value)
        }
    }
}

fn encode_array(types: &[Ty], value: &Json) -> Result<Value, JsonError> {
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

    Ok(Value::Composite(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($abi:expr, $data:expr) => {{
            let json = json!({"abi": $abi, "data": $data });

            let encoded = encode_calldata(&json).unwrap();
            let decoded = decode_calldata(&encoded).unwrap();

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
