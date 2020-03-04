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
        todo!()
    }
}

impl AppDeserializer for AppJsonDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<App> {
        todo!()
    }
}
