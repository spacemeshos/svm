use crate::traits::CodeHashStore;
use crate::types::{Address, CodeHash, Revision, Tag};

pub enum DeployError {
    MissingField(Field),
    EmptyField(Field),
    DependencyNotFound(Revision),
}

pub enum Field {
    CodeHash,
    Tag,
    Name,
    Authors,
    Admins,
    Deps,
}

pub fn deploy_contract(bytes: &[u8], store: &mut impl CodeHashStore) -> Result<(), DeployError> {
    return Ok(());
}
