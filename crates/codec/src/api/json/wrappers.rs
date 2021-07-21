use serde::{Deserialize, Deserializer, Serialize, Serializer};
use svm_types::Address;

/// A blob of binary data that is encoded with Base16.
#[derive(Clone, Debug)]
pub struct HexBlob(pub Vec<u8>);

impl Serialize for HexBlob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(hex::encode_upper(&self.0).as_str())
    }
}

impl<'de> Deserialize<'de> for HexBlob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s: String = Deserialize::deserialize(deserializer)?;
        if s.len() % 2 != 0 {
            return Err(D::Error::custom("Bad length"));
        }
        hex::decode(s)
            .map(|bytes| Self(bytes))
            .map_err(|_| D::Error::custom("Bad hex"))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressWrapper(pub HexBlob);

impl AddressWrapper {
    pub fn address(&self) -> Address {
        Address::from(&self.0 .0[..])
    }
}
