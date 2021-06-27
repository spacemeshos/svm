use std::fmt;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Field {
    Section,
    SectionKind,
    SectionByteSize,
    SectionCount,
    String,
    GasUsed,
    GasMode,
    DeployerAddr,
    Creator,
    Version,
    Description,
    Name,
    State,
    Code,
    CodeKind,
    CodeFlags,
    CodeSize,
    CodeVersion,
    SvmVersion,
    TransactionId,
    Nonce,
    Layer,
    Address,
    TemplateAddr,
    AppAddr,
    CallDataLength,
    CallData,
    LayoutKind,
    LayoutCount,
    LayoutFirstVarId,
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
