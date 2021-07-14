//! Runtime traits

mod address_locator;
mod hasher;
mod serialize;
mod store;

pub use address_locator::ComputeAddress;
pub use hasher::TemplateHasher;
pub use serialize::{AppDeserializer, AppSerializer};
pub use serialize::{TemplateDeserializer, TemplateSerializer};
pub use store::{AppStore, TemplateStore};
