use crate::{String, ToString};

/// Represents a decimal digit.
#[repr(transparent)]
pub struct DecDigit(pub u8);

/// Represents an hex digit.
#[repr(transparent)]
pub struct HexDigit(pub u8);

impl ToString for HexDigit {
    fn to_string(&self) -> String {
        let value = self.0;
        debug_assert!(value < 16);

        if value < 10 {
            let digit = DecDigit(value);
            return digit.to_string();
        }

        // TODO: is possible to use `seq-macro` here?
        match value {
            10 => String::from_byte(b'A'),
            11 => String::from_byte(b'B'),
            12 => String::from_byte(b'C'),
            13 => String::from_byte(b'D'),
            14 => String::from_byte(b'E'),
            15 => String::from_byte(b'F'),
            _ => crate::panic(),
        }
    }
}

impl ToString for DecDigit {
    fn to_string(&self) -> String {
        let value = self.0;
        debug_assert!(value < 10);

        // TODO: is possible to use `seq-macro` here?
        match value {
            0 => String::from_byte(b'0'),
            1 => String::from_byte(b'1'),
            2 => String::from_byte(b'2'),
            3 => String::from_byte(b'3'),
            4 => String::from_byte(b'4'),
            5 => String::from_byte(b'5'),
            6 => String::from_byte(b'6'),
            7 => String::from_byte(b'7'),
            8 => String::from_byte(b'8'),
            9 => String::from_byte(b'9'),
            _ => crate::panic(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_decimal(digit: u8, expected: &'static str) {
        assert_eq!(DecDigit(digit).to_string().to_std_string(), expected);
    }

    fn test_hex(digit: u8, expected: &'static str) {
        assert_eq!(HexDigit(digit).to_string().to_std_string(), expected);
    }

    #[test]
    fn decimal_digit_to_string_sanity() {
        test_decimal(0, "0");
        test_decimal(1, "1");
        test_decimal(2, "2");
        test_decimal(3, "3");
        test_decimal(4, "4");
        test_decimal(5, "5");
        test_decimal(6, "6");
        test_decimal(7, "7");
        test_decimal(8, "8");
        test_decimal(9, "9");
    }

    #[test]
    fn hex_digit_to_string_sanity() {
        test_hex(0, "0");
        test_hex(1, "1");
        test_hex(2, "2");
        test_hex(3, "3");
        test_hex(4, "4");
        test_hex(5, "5");
        test_hex(6, "6");
        test_hex(7, "7");
        test_hex(8, "8");
        test_hex(9, "9");
        test_hex(10, "A");
        test_hex(11, "B");
        test_hex(12, "C");
        test_hex(13, "D");
        test_hex(14, "E");
        test_hex(15, "F");
    }
}
