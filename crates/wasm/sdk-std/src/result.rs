/// Fixed-Gas replacement for [`std::result::Result`].
#[doc(hidden)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[doc(hidden)]
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
            Self::Err(..) => crate::panic(),
        }
    }

    #[inline]
    pub fn unwrap_err(self) -> E {
        match self {
            Self::Err(err) => err,
            Self::Ok(..) => crate::panic(),
        }
    }
}
