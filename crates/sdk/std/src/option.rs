use core::cmp::{Eq, PartialEq};

use crate::Result;
pub enum Option<T> {
    None,

    Some(T),
}

impl<T> Option<T> {
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::None => panic!(),
            Self::Some(val) => val,
        }
    }

    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Self::Some(val) => Result::Ok(val),
            Self::None => Result::Err(err),
        }
    }
}

impl<T: PartialEq> PartialEq for Option<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::None, Self::None) => true,
            (Self::None, Self::Some(..)) => false,
            (Self::Some(..), Self::None) => false,
            (Self::Some(a), Option::Some(b)) => a.eq(b),
        }
    }
}

impl<T: Eq> Eq for Option<T> {}

impl<T: Clone> Clone for Option<T> {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Some(val) => Self::Some(val.clone()),
        }
    }
}

#[cfg(any(test, feature = "debug"))]
impl<T: core::fmt::Debug> core::fmt::Debug for Option<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Some(val) => write!(f, "Some({:?})", val),
        }
    }
}

impl<T> From<Option<T>> for core::option::Option<T> {
    #[inline]
    fn from(value: Option<T>) -> Self {
        match value {
            Option::None => None,
            Option::Some(val) => Some(val),
        }
    }
}

impl<T> From<core::option::Option<T>> for Option<T> {
    #[inline]
    fn from(value: core::option::Option<T>) -> Self {
        match value {
            None => Option::None,
            Some(val) => Option::Some(val),
        }
    }
}
