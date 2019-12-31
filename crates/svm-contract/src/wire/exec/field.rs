pub enum Field {
    Version,
    AppTemplate,
    App,
    Sender,
    FuncNameLength,
    FuncName,
    ArgsCount,
    ArgLength,
    ArgType,
    ArgValue,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let field = match self {
            Field::Version => "Version",
            Field::AppTemplate => "AppTemplate",
            Field::App => "App",
            Field::Sender => "Sender",
            Field::FuncNameLength => "FuncNameLength",
            Field::FuncName => "FuncName",
            Field::ArgsCount => "ArgsCount",
            Field::ArgLength => "ArgLength",
            Field::ArgType => "ArgType",
            Field::ArgValue => "ArgValue",
        };

        write!(f, "{}", field)
    }
}
