use core::cmp::{Ordering, PartialOrd};
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use svm_sdk_std::ensure;

#[derive(PartialEq, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Amount(pub u64);

impl From<i64> for Amount {
    fn from(v: i64) -> Amount {
        Amount(v as _)
    }
}

#[cfg(any(test, feature = "debug"))]
impl core::fmt::Debug for Amount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Amount({})", self.0)
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<u64> for Amount {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub for Amount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ensure!(self.0 >= rhs.0);

        Self(self.0 - rhs.0)
    }
}

impl Sub<u64> for Amount {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        ensure!(self.0 >= rhs);

        Self(self.0 - rhs)
    }
}

impl Mul for Amount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Amount(self.0 * rhs.0)
    }
}

impl Mul<u64> for Amount {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Amount(self.0 * rhs)
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        *self = Amount(self.0 + rhs.0)
    }
}

impl AddAssign<u64> for Amount {
    fn add_assign(&mut self, rhs: u64) {
        *self = Amount(self.0 + rhs)
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        ensure!(self.0 >= rhs.0);

        *self = Amount(self.0 - rhs.0)
    }
}

impl SubAssign<u64> for Amount {
    fn sub_assign(&mut self, rhs: u64) {
        ensure!(self.0 >= rhs);

        *self = Amount(self.0 - rhs)
    }
}

impl MulAssign for Amount {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Amount(self.0 * rhs.0)
    }
}

impl MulAssign<u64> for Amount {
    fn mul_assign(&mut self, rhs: u64) {
        *self = Amount(self.0 * rhs)
    }
}

impl PartialOrd for Amount {
    #[inline]
    fn partial_cmp(&self, other: &Amount) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_add() {
        let a = Amount(10);
        let b = Amount(20);
        let c = Amount(10 + 20);

        assert_eq!(a + b, c);

        let mut d = c;
        d += Amount(5);
        assert_eq!(d, Amount(10 + 20 + 5));

        let e = d + 7;
        assert_eq!(e, Amount(10 + 20 + 5 + 7));

        let mut f = e;
        f += 8;
        assert_eq!(f, Amount(10 + 20 + 5 + 7 + 8));
    }

    #[test]
    fn amount_sub() {
        let a = Amount(100);
        let b = Amount(20);
        let c = Amount(100 - 20);

        assert_eq!(a - b, c);

        let mut d = c;
        d -= Amount(5);
        assert_eq!(d, Amount(100 - 20 - 5));

        let e = d - 7;
        assert_eq!(e, Amount(100 - 20 - 5 - 7));

        let mut f = e;
        f -= 8;
        assert_eq!(f, Amount(100 - 20 - 5 - 7 - 8));
    }

    #[test]
    fn amount_mul() {
        let a = Amount(2);
        let b = Amount(3);
        let c = Amount(2 * 3);

        assert_eq!(a * b, c);

        let mut d = c;
        d *= Amount(4);
        assert_eq!(d, Amount(2 * 3 * 4));

        let e = d * 5;
        assert_eq!(e, Amount(2 * 3 * 4 * 5));

        let mut f = e;
        f *= 6;
        assert_eq!(f, Amount(2 * 3 * 4 * 5 * 6));
    }

    #[test]
    fn amount_partial_ord() {
        let a = Amount(20);
        let b = Amount(10);

        assert!(a == a);
        assert!(a >= a);
        assert!(a <= a);

        assert!(a >= b);
        assert!(a > b);

        assert!(b <= a);
        assert!(b < a);
    }
}
