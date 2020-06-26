use svm_codec::serializers::{
    AppDeserializer, AppSerializer, AppTemplateDeserializer, AppTemplateSerializer,
};

/// `Env` storage serialization types
pub trait EnvSerializerTypes {
    /// `AppTemplate`'s Serializer
    type TemplateSerializer: AppTemplateSerializer;

    /// `AppTemplate`'s Deserializer
    type TemplateDeserializer: AppTemplateDeserializer;

    /// `App`'s Serializer
    type AppSerializer: AppSerializer;

    /// `App`'s Deserializer
    type AppDeserializer: AppDeserializer;
}
