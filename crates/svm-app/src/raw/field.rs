use std::fmt;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Field {
    Author,
    Creator,
    Version,
    NameLength,
    Name,
    PageCount,
    CodeLength,
    Code,
    AppTemplate,
    App,
    FuncIndex,
    FuncBufLength,
    FuncBuf,
    FuncArgsNoMoreMark,
}

impl fmt::Display for Field {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, fmt)
    }
}
