use core::cmp::{Ordering, PartialOrd};
use core::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Amount(pub u64);

impl From<i64> for Amount {
    fn from(v: i64) -> Amount {
        Amount(v as _)
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Amount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.0 >= rhs.0);

        Self(self.0 - rhs.0)
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        *self = Amount(self.0 + rhs.0)
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        assert!(self.0 >= rhs.0);

        *self = Amount(self.0 - rhs.0)
    }
}

impl PartialOrd for Amount {
    #[inline]
    fn partial_cmp(&self, other: &Amount) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
