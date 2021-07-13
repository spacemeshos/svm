//! Runtime traits

mod compute_address;
mod hasher;
mod serialize;
mod store;

pub use compute_address::AddressLocator;
pub use hasher::TemplateHasher;
pub use store::{AppStore, TemplateStore};

pub use serialize::{AppDeserializer, AppSerializer};
pub use serialize::{TemplateDeserializer, TemplateSerializer};
