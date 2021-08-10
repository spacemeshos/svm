use crate::panic;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    #[inline]
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(..) => true,
            Self::Err(..) => false,
        }
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        !(self.is_ok())
    }

    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(value) => value,
            Self::Err(..) => panic(),
        }
    }

    #[inline]
    pub fn unwrap_err(self) -> E {
        match self {
            Self::Err(err) => err,
            Self::Ok(..) => panic(),
        }
    }
}
