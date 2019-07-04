use crate::utils::{u32_to_le_array, u8_pair_add, u8_triple_add};
use std::ops::Add;

/// _Spacemesh_ account address are 32 bytes
/// An `Address` is always assumed to be in a **little-endian** order
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Address(pub [u8; 32]);

/// Should be used **only** for tests
impl From<u32> for Address {
    fn from(n: u32) -> Address {
        let mut addr = [0; 32];

        let [n0, n1, n2, n3] = u32_to_le_array(n);

        addr[0] = n0;
        addr[1] = n1;
        addr[2] = n2;
        addr[3] = n3;

        Address(addr)
    }
}

/// Should be used **only** for tests
impl From<u64> for Address {
    fn from(n: u64) -> Address {
        use crate::utils::u64_to_le_array;

        let mut addr = [0; 32];

        let [n0, n1, n2, n3, n4, n5, n6, n7] = u64_to_le_array(n);

        addr[0] = n0;
        addr[1] = n1;
        addr[2] = n2;
        addr[3] = n3;
        addr[4] = n4;
        addr[5] = n5;
        addr[6] = n6;
        addr[7] = n7;

        Address(addr)
    }
}

impl Add<u32> for Address {
    type Output = [u8; 33];

    fn add(self, b: u32) -> Self::Output {
        // `self.0` is an address (little-endian)  `a0 (lsb), a1, a2, a3, a4, ..., a30, a31 (msb)`
        // `b` is a 4-byte number (little-endian): `b0 (lsb), b1, b2, b3 (msb)`
        //
        // output (marked as `c`) will consist of `33 bytes` and not `32 bytes` since a carry might be turned-on
        // `c0 (lsb), c1, c2, c3, c4, ..., c30, c31, c32 (msb)`

        let mut c = [0; 33];
        c[0..32].copy_from_slice(&self.0);

        let a0 = self.0[0];
        let a1 = self.0[1];
        let a2 = self.0[2];
        let a3 = self.0[3];
        let a4 = self.0[4];

        let [b0, b1, b2, b3] = u32_to_le_array(b);

        let (carry0, c0) = u8_pair_add(a0, b0);
        let (carry1, c1) = u8_triple_add(carry0, a1, b1);
        let (carry2, c2) = u8_triple_add(carry1, a2, b2);
        let (carry3, c3) = u8_triple_add(carry2, a3, b3);
        let (carry4, c4) = u8_pair_add(carry3, a4); // we use `u8_pair_add` and not `u8_triple_add` since `b4 = 0`

        c[0] = c0;
        c[1] = c1;
        c[2] = c2;
        c[3] = c3;
        c[4] = c4;
        c[5] = carry4;

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_from_u32() {
        let expected = Address([
            0x44, 0x33, 0x22, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);

        let actual = Address::from(0x11_22_33_44 as u32);

        assert_eq!(expected, actual);
    }

    #[test]
    fn address_add_u32_no_carry() {
        let expected = [
            0x45, 0x35, 0x25, 0x15, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let actual = Address::from(0x11_22_33_44 as u32).add(0x04_03_02_01);

        // `assert_eq!` isn't implemented for arrays so we compare slices
        assert_eq!(expected[..], actual[..]);
    }

    #[test]
    fn address_add_u32_carry_0() {
        let expected = [
            0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let actual = Address::from(0x00_00_00_FF as u32).add(0x00_00_00_03);

        assert_eq!(expected[..], actual[..]);
    }

    #[test]
    fn address_add_u32_carry_1() {
        let expected = [
            0x00, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let actual = Address::from(0x00_00_FF_00 as u32).add(0x00_00_03_00);

        assert_eq!(expected[..], actual[..]);
    }

    #[test]
    fn address_add_u32_carry_2() {
        let expected = [
            0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let actual = Address::from(0x00_FF_00_00 as u32).add(0x00_03_00_00);

        assert_eq!(expected[..], actual[..]);
    }

    #[test]
    fn address_add_u32_carry_3() {
        let expected = [
            0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let actual = Address::from(0xFF_00_00_00 as u32).add(0x03_00_00_00);

        assert_eq!(expected[..], actual[..]);
    }

    #[test]
    fn address_add_u32_carry_4() {
        let expected = [
            0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let actual = Address::from(0xFF_FF_00_00_00 as u64).add(0x03_00_00_00);

        assert_eq!(expected[..], actual[..]);
    }
}
