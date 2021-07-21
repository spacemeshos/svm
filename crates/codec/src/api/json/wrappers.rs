use serde::{Deserialize, Deserializer, Serialize, Serializer};

use svm_types::Address;

/// A blob of binary data that is encoded with Base16.
#[derive(Clone, Debug)]
pub struct HexBlob<T>(pub T);

impl<T> Serialize for HexBlob<T>
where
    T: AsRef<[u8]>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(hex::encode_upper(&self.0).as_str())
    }
}

impl<'de> Deserialize<'de> for HexBlob<Vec<u8>> {
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

#[derive(Clone, Debug)]
pub struct AddressWrapper(pub Address);

impl Serialize for AddressWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let blob = HexBlob(self.0.as_slice());
        blob.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AddressWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let blob = HexBlob::deserialize(deserializer)?;

        if blob.0.len() != Address::len() {
            Err(D::Error::custom("Bad length"))
        } else {
            Ok(Self(Address::from(&blob.0[..])))
        }
    }
}
