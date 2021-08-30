//! Reusable implementors of [`serde::Serialize`] and [`serde::Deserialize`].

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use svm_types::{Address, BytesPrimitive, TemplateAddr};

/// A blob of binary data that is encoded via Base16.
#[derive(Clone, Debug)]
pub struct HexBlob<T>(pub T);

impl<T> Serialize for HexBlob<T>
where
    T: AsRef<[u8]>,
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(hex::encode_upper(&self.0).as_str())
    }
}

impl<'de> Deserialize<'de> for HexBlob<Vec<u8>> {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s: String = Deserialize::deserialize(de)?;
        hex::decode(s)
            .map(|bytes| Self(bytes))
            .map_err(|_| D::Error::custom("Bad hex"))
    }
}

#[derive(Clone, Debug, derive_more::Into, derive_more::From)]
pub struct AddressWrapper(pub Address);

#[derive(Clone, Debug, derive_more::Into, derive_more::From)]
pub struct TemplateAddrWrapper(TemplateAddr);

impl Serialize for AddressWrapper {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let blob = HexBlob(self.0.as_slice());
        blob.serialize(s)
    }
}

impl<'de> Deserialize<'de> for AddressWrapper {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let blob = HexBlob::deserialize(de)?;

        if blob.0.len() != Address::N {
            Err(D::Error::custom("Bad length"))
        } else {
            Ok(Self(Address::new(&blob.0[..])))
        }
    }
}

impl Serialize for TemplateAddrWrapper {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let blob = HexBlob(self.0.as_slice());
        blob.serialize(s)
    }
}

impl<'de> Deserialize<'de> for TemplateAddrWrapper {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let blob = HexBlob::deserialize(de)?;

        if blob.0.len() != TemplateAddr::N {
            Err(D::Error::custom("Bad length"))
        } else {
            Ok(Self(TemplateAddr::new(&blob.0[..])))
        }
    }
}
