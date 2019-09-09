use super::field::Field;

#[allow(missing_docs)]
pub enum ContractBuildError {
    EmptyName,
    NameNotValidUTF8String,
    DepsNotSupportedYet,
    AdminsNotSupportedYet,
    NotEnoughBytes(Field),
    UnsupportedProtoVersion(u32),
    NoAuthors,
    InvalidWasm,
}

impl std::error::Error for ContractBuildError {
    fn description(&self) -> &'static str {
        match self {
            ContractBuildError::EmptyName => "Name must not be empty",
            ContractBuildError::NameNotValidUTF8String => "Name must be a valid UTF-8 string",
            ContractBuildError::DepsNotSupportedYet => "Dependencies are supported yet",
            ContractBuildError::AdminsNotSupportedYet => "Admins are not supported yet",
            ContractBuildError::NotEnoughBytes(_) => "Not enough bytes",
            ContractBuildError::UnsupportedProtoVersion(_) => "Unsupported protocol version",
            ContractBuildError::NoAuthors => "Must have authors",
            ContractBuildError::InvalidWasm => "Invalid wasm format",
        }
    }
}

impl std::fmt::Display for ContractBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            ContractBuildError::EmptyName => String::from("Name must not be empty"),
            ContractBuildError::NameNotValidUTF8String => {
                String::from("Name must be a valid UTF-8 string")
            }
            ContractBuildError::DepsNotSupportedYet => {
                String::from("Dependencies are supported yet")
            }
            ContractBuildError::AdminsNotSupportedYet => {
                String::from("Admins are not supported yet")
            }
            ContractBuildError::NotEnoughBytes(field) => {
                format!("Not enough bytes (field: {})", field)
            }
            ContractBuildError::UnsupportedProtoVersion(ver) => {
                format!("Unsupported protocol version: `{}`", ver)
            }
            ContractBuildError::NoAuthors => String::from("Must have Authors"),
            ContractBuildError::InvalidWasm => String::from("Invalid wasm format"),
        };

        write!(f, "{}", msg)
    }
}

impl std::fmt::Debug for ContractBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
