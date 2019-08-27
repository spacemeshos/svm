use super::field::Field;

pub enum ContractDeployError {
    EmptyName,
    NameNotValidUTF8String,
    DepsNotSupportedYet,
    AdminsNotSupportedYet,
    NotEnoughBytes(Field),
    UnsupportedProtoVersion(u32),
    NoAuthors,
    InvalidWasm,
}

impl std::error::Error for ContractDeployError {
    fn description(&self) -> &'static str {
        match self {
            ContractDeployError::EmptyName => "Name must not be empty",
            ContractDeployError::NameNotValidUTF8String => "Name must be a valid UTF-8 string",
            ContractDeployError::DepsNotSupportedYet => "Dependencies are supported yet",
            ContractDeployError::AdminsNotSupportedYet => "Admins are not supported yet",
            ContractDeployError::NotEnoughBytes(_) => "Not enough bytes",
            ContractDeployError::UnsupportedProtoVersion(_) => "Unsupported protocol version",
            ContractDeployError::NoAuthors => "Must have authors",
            ContractDeployError::InvalidWasm => "Invalid wasm format",
        }
    }
}

impl std::fmt::Display for ContractDeployError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            ContractDeployError::EmptyName => String::from("Name must not be empty"),
            ContractDeployError::NameNotValidUTF8String => {
                String::from("Name must be a valid UTF-8 string")
            }
            ContractDeployError::DepsNotSupportedYet => {
                String::from("Dependencies are supported yet")
            }
            ContractDeployError::AdminsNotSupportedYet => {
                String::from("Admins are not supported yet")
            }
            ContractDeployError::NotEnoughBytes(field) => {
                String::from(format!("Not enough bytes (field: {})", field))
            }
            ContractDeployError::UnsupportedProtoVersion(ver) => {
                String::from(format!("Unsupported protocol version: `{}`", ver))
            }
            ContractDeployError::NoAuthors => String::from("Must have Authors"),
            ContractDeployError::InvalidWasm => String::from("Invalid wasm format"),
        };

        write!(f, "{}", msg)
    }
}

impl std::fmt::Debug for ContractDeployError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
