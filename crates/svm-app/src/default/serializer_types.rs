use crate::{
    raw::{
        DefaultAppDeserializer, DefaultAppSerializer, DefaultAppTemplateDeserializer,
        DefaultAppTemplateSerializer,
    },
    traits::EnvSerializerTypes,
};

/// Default serializers for `Env`
pub struct DefaultSerializerTypes;

impl EnvSerializerTypes for DefaultSerializerTypes {
    type TemplateSerializer = DefaultAppTemplateSerializer;

    type TemplateDeserializer = DefaultAppTemplateDeserializer;

    type AppSerializer = DefaultAppSerializer;

    type AppDeserializer = DefaultAppDeserializer;
}
