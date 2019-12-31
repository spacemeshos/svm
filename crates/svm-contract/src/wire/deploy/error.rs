use super::field::Field;

#[allow(missing_docs)]
pub enum AppTemplateBuildError {
    EmptyName,
    NameNotValidUTF8String,
    DepsNotSupportedYet,
    AdminsNotSupportedYet,
    NotEnoughBytes(Field),
    UnsupportedProtoVersion(u32),
    NoAuthors,
    InvalidWasm,
}

impl std::error::Error for AppTemplateBuildError {
    fn description(&self) -> &'static str {
        match self {
            AppTemplateBuildError::EmptyName => "Name must not be empty",
            AppTemplateBuildError::NameNotValidUTF8String => "Name must be a valid UTF-8 string",
            AppTemplateBuildError::DepsNotSupportedYet => "Dependencies are supported yet",
            AppTemplateBuildError::AdminsNotSupportedYet => "Admins are not supported yet",
            AppTemplateBuildError::NotEnoughBytes(_) => "Not enough bytes",
            AppTemplateBuildError::UnsupportedProtoVersion(_) => "Unsupported protocol version",
            AppTemplateBuildError::NoAuthors => "Must have authors",
            AppTemplateBuildError::InvalidWasm => "Invalid wasm format",
        }
    }
}

impl std::fmt::Display for AppTemplateBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            AppTemplateBuildError::EmptyName => String::from("Name must not be empty"),
            AppTemplateBuildError::NameNotValidUTF8String => {
                String::from("Name must be a valid UTF-8 string")
            }
            AppTemplateBuildError::DepsNotSupportedYet => {
                String::from("Dependencies are supported yet")
            }
            AppTemplateBuildError::AdminsNotSupportedYet => {
                String::from("Admins are not supported yet")
            }
            AppTemplateBuildError::NotEnoughBytes(field) => {
                format!("Not enough bytes (field: {})", field)
            }
            AppTemplateBuildError::UnsupportedProtoVersion(ver) => {
                format!("Unsupported protocol version: `{}`", ver)
            }
            AppTemplateBuildError::NoAuthors => String::from("Must have Authors"),
            AppTemplateBuildError::InvalidWasm => String::from("Invalid wasm format"),
        };

        write!(f, "{}", msg)
    }
}

impl std::fmt::Debug for AppTemplateBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
