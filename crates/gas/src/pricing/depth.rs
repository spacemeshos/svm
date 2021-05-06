use std::cmp::{Ordering, PartialEq};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Depth(pub u32);

impl Add for Depth {
    type Output = Depth;

    fn add(self, rhs: Self) -> Self::Output {
        Depth(self.0 + rhs.0)
    }
}

impl Sub for Depth {
    type Output = Depth;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.0 >= rhs.0);

        Depth(self.0 - rhs.0)
    }
}

impl Sub<usize> for Depth {
    type Output = Depth;

    fn sub(self, rhs: usize) -> Self::Output {
        assert!(self.0 >= rhs as u32);

        Depth(self.0 - rhs as u32)
    }
}

impl Sub<u32> for Depth {
    type Output = Depth;

    fn sub(self, rhs: u32) -> Self::Output {
        assert!(self.0 >= rhs);

        Depth(self.0 - rhs)
    }
}

impl AddAssign<usize> for Depth {
    fn add_assign(&mut self, rhs: usize) {
        *self = Depth(self.0 + rhs as u32);
    }
}

impl SubAssign<usize> for Depth {
    fn sub_assign(&mut self, rhs: usize) {
        assert!(self.0 >= rhs as u32);

        *self = Depth(self.0 - rhs as u32);
    }
}

impl PartialOrd<Depth> for Depth {
    fn partial_cmp(&self, other: &Depth) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_add() {
        let d1 = Depth(10);
        let d2 = Depth(20);
        let d3 = d1 + d2;

        assert_eq!(d3, Depth(30));
    }

    #[test]
    fn depth_sub() {
        let d1 = Depth(20);
        let d2 = Depth(5);
        let d3 = d1 - d2;

        assert_eq!(d3, Depth(15));
    }

    #[test]
    fn depth_add_assign_usize() {
        let mut d1 = Depth(20);
        d1 += 10usize;

        assert_eq!(d1, Depth(30));
    }

    #[test]
    fn depth_sub_assign_usize() {
        let mut d1 = Depth(20);
        d1 -= 5usize;

        assert_eq!(d1, Depth(15));
    }

    #[test]
    fn depth_partial_cmp() {
        let d1 = Depth(20);
        let d2 = Depth(10);

        assert!(d1 == d1);
        assert!(d2 == d2);
        assert!(d1 != d2);

        assert!(d1 > d2);
        assert!(d1 >= d2);

        assert!(d2 < d1);
        assert!(d2 <= d1);
    }
}
