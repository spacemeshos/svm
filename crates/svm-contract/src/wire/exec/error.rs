use super::field::Field;

pub enum TransactionBuildError {
    UnsupportedProtoVersion(u32),
    NotEnoughBytes(Field),
    FuncNameNotValidUTF8String,
    EmptyFuncName,
    InvalidArgType(u8),
    InvalidArgIntType,
}

impl std::error::Error for TransactionBuildError {
    fn description(&self) -> &'static str {
        match self {
            TransactionBuildError::EmptyFuncName => "Func Name must not be empty",
            TransactionBuildError::FuncNameNotValidUTF8String => {
                "Func Name must be a valid UTF-8 string"
            }
            TransactionBuildError::NotEnoughBytes(_) => "Not enough bytes",
            TransactionBuildError::UnsupportedProtoVersion(_) => "Unsupported protocol version",
            TransactionBuildError::InvalidArgType(_) => "Invalid arg type",
            TransactionBuildError::InvalidArgIntType => "Invalid arg int-type",
        }
    }
}

impl std::fmt::Display for TransactionBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <Self as std::error::Error>::description(self))
    }
}

impl std::fmt::Debug for TransactionBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
