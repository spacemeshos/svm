use super::field::Field;

pub enum ContractError {
    EmptyName,
    NameNotValidUTF8String,
    DepsNotSupportedYet,
    AdminsNotSupportedYet,
    NotEnoughBytes(Field),
    UnsupportedProtoVersion(u32),
    NoAuthors,
    InvalidWasm,
}

impl std::error::Error for ContractError {
    fn description(&self) -> &'static str {
        match self {
            ContractError::EmptyName => "Name must not be empty",
            ContractError::NameNotValidUTF8String => "Name must be a valid UTF-8 string",
            ContractError::DepsNotSupportedYet => "Dependencies are supported yet",
            ContractError::AdminsNotSupportedYet => "Admins are not supported yet",
            ContractError::NotEnoughBytes(_) => "Not enough bytes",
            ContractError::UnsupportedProtoVersion(_) => "Unsupported protocol version",
            ContractError::NoAuthors => "Must have authors",
            ContractError::InvalidWasm => "Invalid wasm format",
        }
    }
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            ContractError::EmptyName => String::from("Name must not be empty"),
            ContractError::NameNotValidUTF8String => {
                String::from("Name must be a valid UTF-8 string")
            }
            ContractError::DepsNotSupportedYet => String::from("Dependencies are supported yet"),
            ContractError::AdminsNotSupportedYet => String::from("Admins are not supported yet"),
            ContractError::NotEnoughBytes(field) => {
                String::from(format!("Not enough bytes (field: {})", field))
            }
            ContractError::UnsupportedProtoVersion(ver) => {
                String::from(format!("Unsupported protocol version: `{}`", ver))
            }
            ContractError::NoAuthors => String::from("Must have Authors"),
            ContractError::InvalidWasm => String::from("Invalid wasm format"),
        };

        write!(f, "{}", msg)
    }
}

impl std::fmt::Debug for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
