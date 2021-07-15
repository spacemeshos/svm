//! Runtime traits

mod address_locator;
mod hasher;
mod serialize;
mod store;

pub use address_locator::ComputeAddress;
pub use hasher::TemplateHasher;
pub use serialize::{AccountDeserializer, AccountSerializer};
pub use serialize::{TemplateDeserializer, TemplateSerializer};
pub use store::{AccountStore, TemplateStore};
