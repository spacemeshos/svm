use crate::{
    traits::{AppDeserializer, AppSerializer},
    types::App,
};

/// Json serializer for `App`
pub struct AppJsonSerializer;

/// Json deserializer for `App`
pub struct AppJsonDeserializer;

impl AppSerializer for AppJsonSerializer {
    fn serialize(app: &App) -> Vec<u8> {
        let s = serde_json::to_string(app).unwrap();
        s.into_bytes()
    }
}

impl AppDeserializer for AppJsonDeserializer {
    fn deserialize(bytes: Vec<u8>) -> Option<App> {
        let s = unsafe { String::from_utf8_unchecked(bytes) };

        serde_json::from_str(s.as_str()).ok()
    }
}
