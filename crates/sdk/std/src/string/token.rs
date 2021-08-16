use crate::ToToken;

/// Represents a Token.
pub enum Token {
    /// Token consisting of a single byte.
    One(u8),

    /// Token consisting of a pair of bytes.
    ///
    /// Examples: `=>`, `==`, `!=`
    Two(u8, u8),
}

/// Represents a decimal digit.
#[repr(transparent)]
pub struct DecDigit(pub u8);

/// Represents an hex digit.
#[repr(transparent)]
pub struct HexDigit(pub u8);

impl ToToken for HexDigit {
    fn to_token(&self) -> Token {
        let value = self.0;
        debug_assert!(value < 16);

        if value < 10 {
            let digit = DecDigit(value);
            return digit.to_token();
        }

        // TODO: is that possible to use `seq-macro` here?
        match value {
            10 => Token::One(b'A'),
            11 => Token::One(b'B'),
            12 => Token::One(b'C'),
            13 => Token::One(b'D'),
            14 => Token::One(b'E'),
            15 => Token::One(b'F'),
            _ => crate::panic(),
        }
    }
}

impl ToToken for DecDigit {
    fn to_token(&self) -> Token {
        let value = self.0;
        debug_assert!(value < 10);

        // TODO: is that possible to use `seq-macro` here?
        match value {
            0 => Token::One(b'0'),
            1 => Token::One(b'1'),
            2 => Token::One(b'2'),
            3 => Token::One(b'3'),
            4 => Token::One(b'4'),
            5 => Token::One(b'5'),
            6 => Token::One(b'6'),
            7 => Token::One(b'7'),
            8 => Token::One(b'8'),
            9 => Token::One(b'9'),
            _ => crate::panic(),
        }
    }
}
