use core::convert::Infallible;

use crate::Result;

/// A macro alternative to the `?` operator. In case cases, it might be
/// preferrable to reduce code size.
#[macro_export]
macro_rules! safe_try {
    ($expr:expr) => {{
        use svm_sdk_std::Result;

        let result = $expr;

        if (result.is_ok()) {
            result.unwrap()
        } else {
            let err = result.unwrap_err();

            return Result::Err(err);
        }
    }};
}

impl<T, E> core::ops::FromResidual<Result<Infallible, E>> for Result<T, E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Result::Ok(..) => crate::panic(),
            Result::Err(err) => Self::Err(err),
        }
    }
}

impl<T, E> core::ops::Try for Result<T, E> {
    type Output = T;
    type Residual = Result<Infallible, E>;

    fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(value) => core::ops::ControlFlow::Continue(value),
            Self::Err(err) => core::ops::ControlFlow::Break(Result::Err(err)),
        }
    }

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }
}
