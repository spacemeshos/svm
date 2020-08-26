use std::fmt;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Field {
    String,
    StringLength,
    GasUsed,
    Author,
    Creator,
    Version,
    NameLength,
    Name,
    CodeSize,
    Code,
    Address,
    TemplateAddr,
    AppAddr,
    CallDataLength,
    CallData,
    DataLayoutVarsCount,
    DataLayoutVarLength,
    ErrorLength,
    FuncNameLength,
    FuncName,
}

impl fmt::Display for Field {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, fmt)
    }
}
