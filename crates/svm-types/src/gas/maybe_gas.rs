use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, AddAssign, Sub};

use crate::gas::OOGError;

/// `MaybeGas` is essentially an `Option<u64>` with extensions
/// to faciliate arithmetic additions and subtractions.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct MaybeGas(Option<u64>);

impl MaybeGas {
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

        MaybeGas(maybe_gas)
    }
}

impl Add<u64> for MaybeGas {
    type Output = MaybeGas;

    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        self.map(|lhs| lhs + rhs)
    }
}

impl AddAssign<u64> for MaybeGas {
    #[inline]
    fn add_assign(&mut self, rhs: u64) {
        *self = self.add(rhs)
    }
}

impl Sub<u64> for MaybeGas {
    type Output = Result<MaybeGas, OOGError>;

    fn sub(self, rhs: u64) -> Self::Output {
        match (self.0, rhs) {
            (None, _) => Ok(MaybeGas::new()),
            (Some(lhs), rhs) => {
                if lhs >= rhs {
                    let maybe_gas = MaybeGas::with(lhs - rhs);

                    Ok(maybe_gas)
                } else {
                    Err(OOGError {})
                }
            }
        }
    }
}

impl PartialOrd<u64> for MaybeGas {
    #[inline]
    fn partial_cmp(&self, rhs: &u64) -> Option<Ordering> {
        match self.0 {
            None => Some(Ordering::Greater),
            Some(lhs) => lhs.partial_cmp(rhs),
        }
    }
}

impl PartialEq<u64> for MaybeGas {
    #[inline]
    fn eq(&self, rhs: &u64) -> bool {
        match self.0 {
            None => false,
            Some(lhs) => lhs.eq(rhs),
        }
    }
}

impl PartialEq<MaybeGas> for u64 {
    #[inline]
    fn eq(&self, rhs: &MaybeGas) -> bool {
        rhs.eq(self)
    }
}

impl From<u64> for MaybeGas {
    #[inline]
    fn from(v: u64) -> Self {
        MaybeGas::with(v)
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
        let gas = MaybeGas::from(10);
        assert_eq!(gas, 10);
    }

    #[test]
    fn maybe_gas_partial_eq() {
        let gas1 = MaybeGas::from(10);
        let gas2 = MaybeGas::from(10);
        let gas3 = MaybeGas::from(30);

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
        let gas = MaybeGas::new();
        assert!(gas.is_none());
        assert!(!gas.is_some());

        assert!(gas > 1);
        assert!(gas >= 1);
        assert!(!(gas < 1));
    }

    #[test]
    fn maybe_gas_partial_ord_some() {
        let gas = MaybeGas::from(10);
        assert!(gas.is_some());
        assert!(!gas.is_none());

        assert!(gas > 9);
        assert!(gas >= 10);
        assert!(!(gas > 10));
    }

    #[test]
    fn maybe_gas_add() {
        let gas = MaybeGas::with(0);
        assert_gas!(gas, 0);

        let gas = gas + 5;
        assert_gas!(gas, 5);

        let gas = gas + 10;
        assert_gas!(gas, 15);
    }

    #[test]
    fn maybe_gas_add_assign() {
        let mut gas = MaybeGas::with(10);
        assert_gas!(gas, 10);

        gas += 20;
        assert_gas!(gas, 30);
    }

    #[test]
    fn maybe_gas_sub() {
        let gas = MaybeGas::with(30);

        let gas = gas - 10;
        assert_gas!(gas.unwrap(), 20);

        let gas = MaybeGas::with(15);
        let gas = gas - 20;
        assert_eq!(Err(OOGError {}), gas);
    }
}
