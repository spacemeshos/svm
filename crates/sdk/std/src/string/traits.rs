use crate::{String, StringBuilder, Token};

pub trait ToString {
    fn to_string(&self) -> String;
}

pub trait ToToken {
    fn to_token(&self) -> Token;
}

impl ToString for bool {
    fn to_string(&self) -> String {
        match *self {
            true => String::new("True"),
            false => String::new("False"),
        }
    }
}

#[repr(transparent)]
pub struct DecDigit(u8);
#[repr(transparent)]
pub struct HexDigit(u8);

impl ToToken for HexDigit {
    fn to_token(&self) -> Token {
        let byte = self.0;
        debug_assert!(byte < 16);

        if byte < 10 {
            let digit = DecDigit(byte);
            return digit.to_token();
        }

        // TODO: is that possible to use `seq-macro` here?
        match byte {
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
        let byte = self.0;
        debug_assert!(byte < 10);

        // TODO: is that possible to use `seq-macro` here?
        match byte {
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

impl ToString for u8 {
    fn to_string(&self) -> String {
        let mut sb = StringBuilder::with_capacity(2);
        let byte = *self;

        if byte < 10 {
            let digit = DecDigit(byte);
            sb.push_token(digit.to_token());
        } else {
            let left = (byte & 0xF0) >> 4;
            let right = byte & 0x0F;

            let left = DecDigit(left).to_token();
            let right = DecDigit(right).to_token();

            sb.push_token(left);
            sb.push_token(right);
        }

        sb.build()
    }
}

impl ToString for i8 {
    fn to_string(&self) -> String {
        let mut sb = StringBuilder::with_capacity(2);
        let mut byte = *self;

        if byte < 0 {
            sb.push_token(Token::One(b'-'));
            byte = -(byte);
        }
        debug_assert!(byte >= 0);

        (byte as u8).to_string()
    }
}

impl ToString for u16 {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl ToString for u32 {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl ToString for u64 {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl ToString for i16 {
    fn to_string(&self) -> String {
        let mut sb = StringBuilder::with_capacity(4);
        let mut num = *self;

        if num < 0 {
            sb.push_token(Token::One(b'-'));
            num = -(num);
        }
        debug_assert!(num >= 0);

        (num as u16).to_string()
    }
}
