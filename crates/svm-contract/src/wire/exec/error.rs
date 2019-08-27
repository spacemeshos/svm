use super::field::Field;

pub enum ContractExecError {
    UnsupportedProtoVersion(u32),
    NotEnoughBytes(Field),
    FuncNameNotValidUTF8String,
    EmptyFuncName,
    InvalidArgType(u8),
}

impl std::error::Error for ContractExecError {
    fn description(&self) -> &'static str {
        match self {
            ContractExecError::EmptyFuncName => "Func Name must not be empty",
            ContractExecError::FuncNameNotValidUTF8String => {
                "Func Name must be a valid UTF-8 string"
            }
            ContractExecError::NotEnoughBytes(_) => "Not enough bytes",
            ContractExecError::UnsupportedProtoVersion(_) => "Unsupported protocol version",
            ContractExecError::InvalidArgType(_) => "Invalid arg type",
        }
    }
}

impl std::fmt::Display for ContractExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <Self as std::error::Error>::description(self))
    }
}

impl std::fmt::Debug for ContractExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
