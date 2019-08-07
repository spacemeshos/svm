use crate::types::{Address, CodeHash, Revision, Tag};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
    code_hash: CodeHash,
    tag: Tag,
    name: String,
    authors: Vec<Address>,
    deps: Vec<Revision>,
}
