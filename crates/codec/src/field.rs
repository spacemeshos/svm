use std::fmt;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Field {
    SectionKind,
    String,
    Section,
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
    RawVarCount,
    RawVarSize,
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
    SymbolicVarCount,
    SymbolicVarType,
    SymbolicVarName,
}

impl fmt::Display for Field {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, fmt)
    }
}
