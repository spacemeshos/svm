use std::fmt;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Field {
    String,
    GasUsed,
    Author,
    Creator,
    Version,
    Name,
    CodeSize,
    State,
    Code,
    Address,
    TemplateAddr,
    AppAddr,
    CallDataLength,
    CallData,
    VarsCount,
    VarLength,
    ErrorLength,
    Function,
    Ctor,
    CtorsCount,
    ReceiptType,
    ReceiptStatus,
    LogsCount,
    LogMessage,
    LogMessageLength,
    LogCode,
}

impl fmt::Display for Field {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, fmt)
    }
}
