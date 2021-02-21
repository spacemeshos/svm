use crate::env::{serialize, traits};

use traits::EnvSerializers;

use serialize::{DefaultAppDeserializer, DefaultAppSerializer};
use serialize::{DefaultTemplateDeserializer, DefaultTemplateSerializer};

/// Default serializers for `Env`
pub struct DefaultSerializers;

impl EnvSerializers for DefaultSerializers {
    type TemplateSerializer = DefaultTemplateSerializer;

    type TemplateDeserializer = DefaultTemplateDeserializer;

    type AppSerializer = DefaultAppSerializer;

    type AppDeserializer = DefaultAppDeserializer;
}
