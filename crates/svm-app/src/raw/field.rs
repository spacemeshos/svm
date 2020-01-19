use std::fmt;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Field {
    Version,
    NameLength,
    Name,
    Admins,
    AdminsCount,
    CodeLength,
    Dependencies,
    DependenciesCount,
    PagesCount,
    Code,
    AppTemplate,
    App,
    FuncNameLength,
    FuncName,
    FuncBufSlicesCount,
    FuncBufSliceLength,
    FuncBufSlice,
    FuncArgsCount,
    WasmType,
    WasmValue,
}

impl fmt::Display for Field {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, fmt)
    }
}
