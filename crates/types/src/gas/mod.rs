use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Sub};

mod error;

pub use error::OOGError;

/// `MaybeGas` is essentially an `Option<u64>` with extensions
/// to faciliate arithmetic additions and subtractions.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct Gas(Option<u64>);

impl Default for Gas {
    fn default() -> Self {
        Self::new()
    }
}

impl Gas {
    /// New `MaybeGas` backed by a `None`
    pub fn new() -> Self {
        Self(None)
    }

    /// New `MaybeGas` backed by a `Some(gas)`
    pub fn with(gas: u64) -> Self {
        Self(Some(gas))
    }

    /// Returns `true` if the gas holds a `Some(u64)`
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Returns `true` if the gas holds a `None`.
    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Returns the wrapped `u64`
    ///
    /// # Panics
    ///
    /// Panics if there is no wrapped `u64`.
    #[inline]
    pub fn unwrap(&self) -> u64 {
        self.0.unwrap()
    }

    /// Returns the wrapped `u64` or `default` when there' no underlying `u64`.
    #[inline]
    pub fn unwrap_or(&self, default: u64) -> u64 {
        self.0.unwrap_or(default)
    }

    /// Maps the underling `u64` in case exists. Otherwise does nothing.
    #[inline]
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(u64) -> u64,
    {
        let maybe_gas = self.0.map(|x| f(x));

        Gas(maybe_gas)
    }
}

impl Add<u64> for Gas {
    type Output = Gas;

    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        self.map(|lhs| lhs + rhs)
    }
}

impl AddAssign<u64> for Gas {
    #[inline]
    fn add_assign(&mut self, rhs: u64) {
        *self = self.add(rhs)
    }
}

impl Sub<u64> for Gas {
    type Output = Result<Gas, OOGError>;

    fn sub(self, rhs: u64) -> Self::Output {
        match (self.0, rhs) {
            (None, _) => Ok(Gas::new()),
            (Some(lhs), rhs) => {
                if lhs >= rhs {
                    let maybe_gas = Gas::with(lhs - rhs);

                    Ok(maybe_gas)
                } else {
                    Err(OOGError {})
                }
            }
        }
    }
}

impl PartialOrd<u64> for Gas {
    #[inline]
    fn partial_cmp(&self, rhs: &u64) -> Option<Ordering> {
        match self.0 {
            None => Some(Ordering::Greater),
            Some(lhs) => lhs.partial_cmp(rhs),
        }
    }
}

impl PartialEq<u64> for Gas {
    #[inline]
    fn eq(&self, rhs: &u64) -> bool {
        match self.0 {
            None => false,
            Some(lhs) => lhs.eq(rhs),
        }
    }
}

impl PartialEq<Gas> for u64 {
    #[inline]
    fn eq(&self, rhs: &Gas) -> bool {
        rhs.eq(self)
    }
}

impl From<u64> for Gas {
    #[inline]
    fn from(v: u64) -> Self {
        Gas::with(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_gas {
        ($maybe_gas:expr, $int:expr) => {{
            assert_eq!($maybe_gas.unwrap(), $int);
        }};
    }

    #[test]
    fn maybe_gas_from() {
        let gas = Gas::from(10);
        assert_eq!(gas, 10);
    }

    #[test]
    fn maybe_gas_partial_eq() {
        let gas1 = Gas::from(10);
        let gas2 = Gas::from(10);
        let gas3 = Gas::from(30);

        assert_eq!(gas1, 10);
        assert_ne!(gas1, 20);

        assert_eq!(10, gas1);
        assert_ne!(20, gas1);

        assert_eq!(gas1, gas1);
        assert_eq!(gas1, gas2);

        assert_ne!(gas1, gas3);
    }

    #[test]
    fn maybe_gas_partial_ord_none() {
        let gas = Gas::new();
        assert!(gas.is_none());
        assert!(!gas.is_some());

        assert!(gas > 1);
        assert!(gas >= 1);
        assert!(!(gas < 1));
    }

    #[test]
    fn maybe_gas_partial_ord_some() {
        let gas = Gas::from(10);
        assert!(gas.is_some());
        assert!(!gas.is_none());

        assert!(gas > 9);
        assert!(gas >= 10);
        assert!(!(gas > 10));
    }

    #[test]
    fn maybe_gas_add() {
        let gas = Gas::with(0);
        assert_gas!(gas, 0);

        let gas = gas + 5;
        assert_gas!(gas, 5);

        let gas = gas + 10;
        assert_gas!(gas, 15);
    }

    #[test]
    fn maybe_gas_add_assign() {
        let mut gas = Gas::with(10);
        assert_gas!(gas, 10);

        gas += 20;
        assert_gas!(gas, 30);
    }

    #[test]
    fn maybe_gas_sub() {
        let gas = Gas::with(30);

        let gas = gas - 10;
        assert_gas!(gas.unwrap(), 20);

        let gas = Gas::with(15);
        let gas = gas - 20;
        assert_eq!(Err(OOGError {}), gas);
    }
}
