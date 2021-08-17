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

        // TODO: is that possible to use `seq-macro` here?
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

        // TODO: is that possible to use `seq-macro` here?
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
