use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Revision(pub [u8; 32]);

#[repr(transparent)]
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct CodeHash(pub [u8; 32]);

#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Tag(pub [u8; 4]);
