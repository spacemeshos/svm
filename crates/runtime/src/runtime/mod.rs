//! Implements the most high-level API of `SVM`.

mod call;
mod function;
mod outcome;
mod runtime;

pub use call::Call;
pub use function::Function;
pub use outcome::Outcome;
pub use runtime::{compute_account_addr, compute_template_addr, Runtime};
