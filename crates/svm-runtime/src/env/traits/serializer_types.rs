use svm_codec::serializers::{
    AppDeserializer, AppSerializer, TemplateDeserializer, TemplateSerializer,
};

/// `Env` storage serialization types
pub trait EnvSerializerTypes {
    /// `Template`'s Serializer
    type TemplateSerializer: TemplateSerializer;

    /// `Template`'s Deserializer
    type TemplateDeserializer: TemplateDeserializer;

    /// `App`'s Serializer
    type AppSerializer: AppSerializer;

    /// `App`'s Deserializer
    type AppDeserializer: AppDeserializer;
}
