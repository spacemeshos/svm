/// `Addressable` types
use derive_more::{AsRef, From};

use crate::BytesPrimitive;

/// The address of an [`Account`](crate::Account).
#[derive(Debug, Default, Copy, Clone, From, Hash, PartialEq, Eq, AsRef)]
pub struct Address(pub [u8; 20]);

impl BytesPrimitive<20> for Address {}

/// The address of a [`Template`](crate::Template).
#[derive(Debug, Default, Copy, Clone, From, Hash, PartialEq, Eq, AsRef)]
pub struct TemplateAddr(pub [u8; 20]);

impl TemplateAddr {
    /// Returns a special-designated [`TemplateAddr`] for accounts created at
    /// genesis.
    pub fn god_template() -> Self {
        Self::zeros()
    }
}

impl BytesPrimitive<20> for TemplateAddr {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_len() {
        assert_eq!(20, Address::N);
        assert_eq!(20, TemplateAddr::N);
    }

    #[test]
    fn address_partial_eq() {
        let addr1 = Address::repeat(0xAB);
        let addr2 = Address::repeat(0xAB);
        let addr3 = Address::repeat(0xCD);

        assert_eq!(addr1, addr2);
        assert_eq!(addr2, addr1);
        assert_ne!(addr1, addr3);
    }

    #[test]
    fn address_from() {
        let expected = Address([
            0x44, 0x33, 0x22, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);

        let addr: Vec<u8> = vec![
            0x44, 0x33, 0x22, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let actual = Address::new(addr);

        assert_eq!(expected, actual);
    }

    #[test]
    fn address_as_slice() {
        let bytes = [
            0x44, 0x33, 0x22, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let addr = Address(bytes);

        assert_eq!(&bytes[..], addr.as_slice());
    }

    #[test]
    fn address_of_str() {
        let bytes: [u8; 20] = [
            b'a', b'd', b'd', b'r', b'e', b's', b's', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let expected = Address::new(&bytes[..]);
        let actual = Address::of("address");

        assert_eq!(expected, actual);
    }

    #[test]
    fn address_fmt_hex() {
        let addr = Address([
            0x10, 0x20, 0x30, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xAA, 0xBB, 0xCC, 0xDD,
        ]);

        assert_eq!("10203040...AABBCCDD", addr.fmt(4, 4));
    }

    #[test]
    fn address_as_str() {
        let addr = Address([
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
            0xF0, 0xAB, 0xBC, 0xCD, 0xDE, 0xEF,
        ]);

        assert_eq!(addr.to_string(), "102030405060708090A0B0C0D0E0F0ABBCCDDEEF");
    }
}
