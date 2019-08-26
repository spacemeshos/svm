pub enum Field {
    Version,
    NameLength,
    Name,
    Author,
    AdminsCount,
    CodeLength,
    DepsCount,
    Code,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let field = match self {
            Field::Version => "Version",
            Field::NameLength => "NameLength",
            Field::Name => "Name",
            Field::Author => "Author",
            Field::AdminsCount => "AdminsCount",
            Field::CodeLength => "CodeLength",
            Field::DepsCount => "DepsCount",
            Field::Code => "Code",
        };

        write!(f, "{}", field)
    }
}
