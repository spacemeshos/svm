use crate::error::ProgramError;
use crate::program::Program;

pub fn validate_wasm(_wasm: &[u8]) -> Result<(), ProgramError> {
    todo!()
}

pub(crate) fn validate_program(_progam: &Program) -> Result<(), ProgramError> {
    todo!()
}
