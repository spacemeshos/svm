mod validation;

pub use validation::ValidationMiddleware;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnsupportedOpcode,
}
