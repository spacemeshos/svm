use crate::{
    raw::{
        AppJsonDeserializer, AppJsonSerializer, AppTemplateJsonDeserializer,
        AppTemplateJsonSerializer,
    },
    traits::EnvSerializerTypes,
};

pub struct DefaultJsonSerializerTypes;

impl EnvSerializerTypes for DefaultJsonSerializerTypes {
    type TemplateSerializer = AppTemplateJsonSerializer;

    type TemplateDeserializer = AppTemplateJsonDeserializer;

    type AppSerializer = AppJsonSerializer;

    type AppDeserializer = AppJsonDeserializer;
}
