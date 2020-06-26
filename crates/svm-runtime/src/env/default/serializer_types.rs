use crate::env::traits::EnvSerializerTypes;

use svm_codec::serializers::{
    DefaultAppDeserializer, DefaultAppSerializer, DefaultAppTemplateDeserializer,
    DefaultAppTemplateSerializer,
};

/// Default serializers for `Env`
pub struct DefaultSerializerTypes;

impl EnvSerializerTypes for DefaultSerializerTypes {
    type TemplateSerializer = DefaultAppTemplateSerializer;

    type TemplateDeserializer = DefaultAppTemplateDeserializer;

    type AppSerializer = DefaultAppSerializer;

    type AppDeserializer = DefaultAppDeserializer;
}
