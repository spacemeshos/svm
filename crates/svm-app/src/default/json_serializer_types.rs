use crate::{
    raw::{
        AppJsonDeserializer, AppJsonSerializer, AppTemplateJsonDeserializer,
        AppTemplateJsonSerializer,
    },
    traits::EnvSerializerTypes,
};

/// Json serializers for `Env`
pub struct DefaultJsonSerializerTypes;

impl EnvSerializerTypes for DefaultJsonSerializerTypes {
    type TemplateSerializer = AppTemplateJsonSerializer;

    type TemplateDeserializer = AppTemplateJsonDeserializer;

    type AppSerializer = AppJsonSerializer;

    type AppDeserializer = AppJsonDeserializer;
}
