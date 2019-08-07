use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Address([u8; 32]);

#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractRev([u8; 32]);

#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeHash([u8; 32]);

#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Tag([u8; 4]);

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
    code_hash: CodeHash,
    tag: Tag,
    name: String,
    authors: Vec<Address>,
    admins: Vec<Address>,
    deps: Vec<ContractRev>,
}
