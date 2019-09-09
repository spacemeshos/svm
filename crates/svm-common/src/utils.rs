/// Converts an unsigned 32-bit integer into a 4-byte array (ordered in Big-Endian)
#[inline(always)]
pub fn u32_to_be_array(num: u32) -> [u8; 4] {
    let b3 = ((num >> 24) & 0xFF) as u8;
    let b2 = ((num >> 16) & 0xFF) as u8;
    let b1 = ((num >> 8) & 0xFF) as u8;
    let b0 = (num & 0xFF) as u8;

    [b3, b2, b1, b0]
}

/// Converts an unsigned 32-bit integer into a 4-byte array (ordered in Little-Endian)
#[inline(always)]
pub fn u32_to_le_array(num: u32) -> [u8; 4] {
    let b0 = ((num >> 24) & 0xFF) as u8;
    let b1 = ((num >> 16) & 0xFF) as u8;
    let b2 = ((num >> 8) & 0xFF) as u8;
    let b3 = (num & 0xFF) as u8;

    [b3, b2, b1, b0]
}

/// Converts an unsigned 64-bit integer into a 8-byte array (ordered in Big-Endian)
#[inline(always)]
pub fn u64_to_be_array(num: u64) -> [u8; 8] {
    let b7 = ((num >> 56) & 0xFF) as u8;
    let b6 = ((num >> 48) & 0xFF) as u8;
    let b5 = ((num >> 40) & 0xFF) as u8;
    let b4 = ((num >> 32) & 0xFF) as u8;
    let b3 = ((num >> 24) & 0xFF) as u8;
    let b2 = ((num >> 16) & 0xFF) as u8;
    let b1 = ((num >> 8) & 0xFF) as u8;
    let b0 = (num & 0xFF) as u8;

    [b7, b6, b5, b4, b3, b2, b1, b0]
}

/// Converts an unsigned 64-bit integer into a 8-byte array (ordered in Little-Endian)
#[inline(always)]
pub fn u64_to_le_array(num: u64) -> [u8; 8] {
    let b0 = ((num >> 56) & 0xFF) as u8;
    let b1 = ((num >> 48) & 0xFF) as u8;
    let b2 = ((num >> 40) & 0xFF) as u8;
    let b3 = ((num >> 32) & 0xFF) as u8;
    let b4 = ((num >> 24) & 0xFF) as u8;
    let b5 = ((num >> 16) & 0xFF) as u8;
    let b6 = ((num >> 8) & 0xFF) as u8;
    let b7 = (num & 0xFF) as u8;

    [b7, b6, b5, b4, b3, b2, b1, b0]
}

/// Adds 2 unsigned bytes and returns also the carry.
///
/// # Example
///
/// u8_pair_add(10, 20)   -> returns (0, 30)
/// u8_pair_add(255, 10)  -> returns (1, 9)
///
#[inline(always)]
pub fn u8_pair_add(a: u8, b: u8) -> (u8, u8) {
    let c = u16::from(a) + u16::from(b);

    let c0 = (c & 0xFF) as u8;
    let c1 = ((c >> 8) & 0xFF) as u8;

    (c1, c0)
}

/// Adds 3 unsigned bytes and returns also the carry.
///
/// u8_triple_add(10, 20, 30)   -> returns (0, 60)
/// u8_triple_add(255, 5, 5)    -> returns (1, 9)
#[inline(always)]
pub fn u8_triple_add(a: u8, b: u8, c: u8) -> (u8, u8) {
    let d = u16::from(a) + u16::from(b) + u16::from(c);

    let d0 = (d & 0xFF) as u8;
    let d1 = ((d >> 8) & 0xFF) as u8;

    (d1, d0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_to_be_array() {
        let expected = [0x11, 0x22, 0x33, 0x44];
        let actual = u32_to_be_array(0x11_22_33_44);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_u32_to_le_array() {
        let expected = [0x44, 0x33, 0x22, 0x11];
        let actual = u32_to_le_array(0x11_22_33_44);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_u64_to_be_array() {
        let expected = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
        let actual = u64_to_be_array(0x11_22_33_44_55_66_77_88);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_u64_to_le_array() {
        let expected = [0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11];
        let actual = u64_to_le_array(0x11_22_33_44_55_66_77_88);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_u8_pair_add_no_carry() {
        assert_eq!((0x00, 0x33), u8_pair_add(0x11, 0x22));
    }

    #[test]
    fn test_u8_pair_add_with_carry() {
        assert_eq!((0x01, 0x03), u8_pair_add(0xFE, 0x05));
    }

    #[test]
    fn test_u8_triple_add_no_carry() {
        assert_eq!((0x00, 0x66), u8_triple_add(0x11, 0x22, 0x33));
    }

    #[test]
    fn test_u8_triple_add_with_carry() {
        assert_eq!((0x01, 0x02), u8_triple_add(0xFE, 0x01, 0x03));
    }
}
