use std::cmp;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Copy, Clone, PartialEq)]
pub enum Gas {
    /// Gas is known-ahead precisely
    Fixed(u64),

    /// Gas is within a range
    Range { min: u64, max: u64 },
}

impl fmt::Debug for Gas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Gas::Fixed(n) => write!(f, "Gas({})", n),
            Gas::Range { min, max } => write!(f, "Gas(min: {}, max: {})", min, max),
        }
    }
}

/// `Add` serves here as a logical `OR`
/// If we have an `if-statement` we either execute the `when true` branch
/// or the `else` branch.
impl Add for Gas {
    type Output = Gas;

    fn add(self, rhs: Gas) -> Self::Output {
        match (self, rhs) {
            (Gas::Fixed(x), Gas::Fixed(y)) if x == y => Gas::Fixed(x),
            (Gas::Fixed(x), Gas::Fixed(y)) => Gas::Range {
                min: cmp::min(x, y),
                max: cmp::max(x, y),
            },
            (Gas::Fixed(x), Gas::Range { min: a, max: b }) => Gas::Range {
                min: cmp::min(a, x),
                max: cmp::max(b, x),
            },
            (Gas::Range { min: a, max: b }, Gas::Fixed(x)) => Gas::Range {
                min: cmp::min(a, x),
                max: cmp::max(b, x),
            },
            (Gas::Range { min: a, max: b }, Gas::Range { min: c, max: d }) => Gas::Range {
                min: cmp::min(a, c),
                max: cmp::max(b, d),
            },
        }
    }
}

/// `Mul` serves here as a logical `AND`
impl Mul for Gas {
    type Output = Gas;

    fn mul(self, rhs: Gas) -> Self::Output {
        match (self, rhs) {
            (Gas::Fixed(x), Gas::Fixed(y)) => Gas::Fixed(x + y),
            (Gas::Fixed(x), Gas::Range { min: a, max: b }) => Gas::Range {
                min: a + x,
                max: b + x,
            },
            (Gas::Range { min: a, max: b }, Gas::Fixed(x)) => Gas::Range {
                min: a + x,
                max: b + x,
            },
            (Gas::Range { min: a, max: b }, Gas::Range { min: c, max: d }) => Gas::Range {
                min: a + c,
                max: b + d,
            },
        }
    }
}

impl MulAssign<Gas> for Gas {
    fn mul_assign(&mut self, rhs: Gas) {
        *self = self.mul(rhs);
    }
}

impl AddAssign<Gas> for Gas {
    fn add_assign(&mut self, rhs: Gas) {
        *self = self.add(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gas_add_fixed_fixed() {
        assert_eq!(Gas::Fixed(0), Gas::Fixed(0) + Gas::Fixed(0));
        assert_eq!(Gas::Fixed(1), Gas::Fixed(1) + Gas::Fixed(1));
        assert_eq!(Gas::Range { min: 1, max: 2 }, Gas::Fixed(1) + Gas::Fixed(2));
        assert_eq!(Gas::Range { min: 1, max: 2 }, Gas::Fixed(2) + Gas::Fixed(1));

        let mut gas = Gas::Fixed(0);
        gas += Gas::Fixed(0);
        assert_eq!(Gas::Fixed(0), gas);

        let mut gas = Gas::Fixed(1);
        gas += Gas::Fixed(1);
        assert_eq!(Gas::Fixed(1), gas);

        let mut gas = Gas::Fixed(1);
        gas += Gas::Fixed(2);
        assert_eq!(Gas::Range { min: 1, max: 2 }, gas);

        let mut gas = Gas::Fixed(2);
        gas += Gas::Fixed(1);
        assert_eq!(Gas::Range { min: 1, max: 2 }, gas);
    }

    #[test]
    fn gas_add_range_fixed() {
        assert_eq!(
            Gas::Range { min: 1, max: 3 },
            Gas::Range { min: 1, max: 2 } + Gas::Fixed(3)
        );

        assert_eq!(
            Gas::Range { min: 1, max: 5 },
            Gas::Range { min: 1, max: 5 } + Gas::Fixed(3)
        );

        let mut gas = Gas::Range { min: 1, max: 2 };
        gas += Gas::Fixed(3);
        assert_eq!(Gas::Range { min: 1, max: 3 }, gas);

        let mut gas = Gas::Range { min: 1, max: 5 };
        gas += Gas::Fixed(3);
        assert_eq!(Gas::Range { min: 1, max: 5 }, gas);
    }

    #[test]
    fn gas_add_fixed_range() {
        assert_eq!(
            Gas::Range { min: 1, max: 3 },
            Gas::Fixed(3) + Gas::Range { min: 1, max: 2 }
        );

        assert_eq!(
            Gas::Range { min: 1, max: 5 },
            Gas::Fixed(3) + Gas::Range { min: 1, max: 5 }
        );

        let mut gas = Gas::Fixed(3);
        gas += Gas::Range { min: 1, max: 2 };
        assert_eq!(Gas::Range { min: 1, max: 3 }, gas);

        let mut gas = Gas::Fixed(3);
        gas += Gas::Range { min: 1, max: 5 };
        assert_eq!(Gas::Range { min: 1, max: 5 }, gas);
    }

    #[test]
    fn gas_add_range_range() {
        assert_eq!(
            Gas::Range { min: 1, max: 4 },
            Gas::Range { min: 1, max: 2 } + Gas::Range { min: 3, max: 4 }
        );

        assert_eq!(
            Gas::Range { min: 1, max: 4 },
            Gas::Range { min: 3, max: 4 } + Gas::Range { min: 1, max: 2 }
        );

        let mut gas = Gas::Range { min: 1, max: 2 };
        gas += Gas::Range { min: 3, max: 4 };
        assert_eq!(Gas::Range { min: 1, max: 4 }, gas);

        let mut gas = Gas::Range { min: 3, max: 4 };
        gas += Gas::Range { min: 1, max: 2 };
        assert_eq!(Gas::Range { min: 1, max: 4 }, gas);
    }

    #[test]
    fn gas_mul_fixed_fixed() {
        assert_eq!(Gas::Fixed(0), Gas::Fixed(0) * Gas::Fixed(0));
        assert_eq!(Gas::Fixed(1 + 1), Gas::Fixed(1) * Gas::Fixed(1));

        assert_eq!(Gas::Fixed(1 + 2), Gas::Fixed(1) * Gas::Fixed(2));
        assert_eq!(Gas::Fixed(1 + 2), Gas::Fixed(2) * Gas::Fixed(1));

        let mut gas = Gas::Fixed(0);
        gas *= Gas::Fixed(0);
        assert_eq!(Gas::Fixed(0), gas);

        let mut gas = Gas::Fixed(1);
        gas *= Gas::Fixed(1);
        assert_eq!(Gas::Fixed(1 + 1), gas);

        let mut gas = Gas::Fixed(1);
        gas *= Gas::Fixed(2);
        assert_eq!(Gas::Fixed(1 + 2), gas);

        let mut gas = Gas::Fixed(2);
        gas *= Gas::Fixed(1);
        assert_eq!(Gas::Fixed(1 + 2), gas);
    }

    #[test]
    fn gas_mul_range_fixed() {
        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 2 + 3
            },
            Gas::Range { min: 1, max: 2 } * Gas::Fixed(3)
        );

        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 5 + 3
            },
            Gas::Range { min: 1, max: 5 } * Gas::Fixed(3)
        );

        let mut gas = Gas::Range { min: 1, max: 2 };
        gas *= Gas::Fixed(3);
        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 2 + 3
            },
            gas
        );

        let mut gas = Gas::Range { min: 1, max: 5 };
        gas *= Gas::Fixed(3);
        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 5 + 3
            },
            gas
        );
    }

    #[test]
    fn gas_mul_fixed_range() {
        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 2 + 3
            },
            Gas::Fixed(3) * Gas::Range { min: 1, max: 2 }
        );

        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 5 + 3
            },
            Gas::Fixed(3) * Gas::Range { min: 1, max: 5 }
        );

        let mut gas = Gas::Fixed(3);
        gas *= Gas::Range { min: 1, max: 2 };
        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 2 + 3
            },
            gas
        );

        let mut gas = Gas::Fixed(3);
        gas *= Gas::Range { min: 1, max: 5 };
        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 5 + 3
            },
            gas
        );
    }

    #[test]
    fn gas_mul_range_range() {
        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 2 + 4
            },
            Gas::Range { min: 1, max: 2 } * Gas::Range { min: 3, max: 4 }
        );

        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 2 + 4
            },
            Gas::Range { min: 3, max: 4 } * Gas::Range { min: 1, max: 2 }
        );

        let mut gas = Gas::Range { min: 1, max: 2 };
        gas *= Gas::Range { min: 3, max: 4 };
        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 2 + 4
            },
            gas
        );

        let mut gas = Gas::Range { min: 3, max: 4 };
        gas *= Gas::Range { min: 1, max: 2 };
        assert_eq!(
            Gas::Range {
                min: 1 + 3,
                max: 2 + 4
            },
            gas
        );
    }
}
