use crate::env::traits::EnvSerializerTypes;

use svm_codec::serializers::{
    DefaultAppDeserializer, DefaultAppSerializer, DefaultTemplateDeserializer,
    DefaultTemplateSerializer,
};

/// Default serializers for `Env`
pub struct DefaultSerializerTypes;

impl EnvSerializerTypes for DefaultSerializerTypes {
    type TemplateSerializer = DefaultTemplateSerializer;

    type TemplateDeserializer = DefaultTemplateDeserializer;

    type AppSerializer = DefaultAppSerializer;

    type AppDeserializer = DefaultAppDeserializer;
}
