use byteorder::{BigEndian, ByteOrder};

/// Spacemesh balance primitive.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Balance(pub u128);

impl From<*const u8> for Balance {
    fn from(balance_ptr: *const u8) -> Balance {
        let slice: &[u8] = unsafe { std::slice::from_raw_parts(balance_ptr, 16) };

        let mut buf: [u8; 16] = [0; 16];
        buf.copy_from_slice(slice);

        let balance = BigEndian::read_u128(&buf);
        Balance(balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_from_ptr() {
        let expected = Balance(
            (1 << 8 * 0)
                + (10 << 8 * 1)
                + (20 << 8 * 2)
                + (30 << 8 * 3)
                + (40 << 8 * 4)
                + (50 << 8 * 5)
                + (60 << 8 * 6)
                + (70 << 8 * 7)
                + (80 << 8 * 8)
                + (90 << 8 * 9)
                + (0x0A << 8 * 10)
                + (0x0B << 8 * 11)
                + (0x0C << 8 * 12)
                + (0x0D << 8 * 13)
                + (0x0E << 8 * 14)
                + (0x0F << 8 * 15),
        );

        let balance_ptr: *const u8 = vec![
            0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 90, 80, 70, 60, 50, 40, 30, 20, 10, 01,
        ]
        .as_ptr();

        let actual = Balance::from(balance_ptr);

        assert_eq!(expected, actual);
    }
}
