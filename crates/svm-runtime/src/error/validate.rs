use svm_app::error::ParseError;
use svm_gas::error::ProgramError;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum ValidateError {
    Parse(ParseError),
    Program(ProgramError),
}

impl From<ParseError> for ValidateError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

impl From<ProgramError> for ValidateError {
    fn from(err: ProgramError) -> Self {
        Self::Program(err)
    }
}
