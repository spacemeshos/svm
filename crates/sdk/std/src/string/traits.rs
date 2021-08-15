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

#[inline]
fn next_digit(value: u64) -> (u8, u64, bool) {
    let digit = value % 10;
    let value = value / 10;
    let completed = value == 0;

    (digit as u8, value, completed)
}

fn num_as_string(num: u64, is_negative: bool) -> String {
    let mut value = num;
    let mut digits = [0u8; 20];
    let mut has_more = true;
    let mut used_count = 0;

    seq_macro::seq!(N in 0..21 {
        if has_more {
            let (digit, new_value, completed) = next_digit(value);

            digits[used_count] = digit;
            used_count += 1;

            value = new_value;
            has_more = !completed;
        }
        else {
            return concat_digits(&digits, used_count, is_negative);
            debug_assert_eq!(value, 0);
        }
    });

    // we should never get here
    crate::panic()
}

fn concat_digits(digits: &[u8; 20], used_count: usize, is_negative: bool) -> String {
    let mut sb = StringBuilder::with_capacity(21);
    if is_negative {
        sb.push_token(Token::One(b'-'));
    }

    seq_macro::seq!(N in 0..20 {
        if N < used_count {
            let digit = digits[N];
            let token = DecDigit(digit).to_token();
            sb.push_token(token);
        }
        else {
            return sb.build()
        }
    });

    sb.build()
}

impl ToString for u8 {
    fn to_string(&self) -> String {
        num_as_string(*self as u64, false)
    }
}

impl ToString for i8 {
    fn to_string(&self) -> String {
        let num = self.abs();
        let is_negative = *self < 0;
        num_as_string(num as u64, is_negative)
    }
}

impl ToString for u16 {
    fn to_string(&self) -> String {
        num_as_string(*self as u64, false)
    }
}

impl ToString for i16 {
    fn to_string(&self) -> String {
        let num = self.abs();
        let is_negative = *self < 0;
        num_as_string(num as u64, is_negative)
    }
}

impl ToString for u32 {
    fn to_string(&self) -> String {
        num_as_string(*self as u64, false)
    }
}

impl ToString for i32 {
    fn to_string(&self) -> String {
        let num = self.abs();
        let is_negative = *self < 0;
        num_as_string(num as u64, is_negative)
    }
}

impl ToString for u64 {
    fn to_string(&self) -> String {
        num_as_string(*self as u64, false)
    }
}

impl ToString for i64 {
    fn to_string(&self) -> String {
        let num = self.abs();
        let is_negative = *self < 0;
        num_as_string(num as u64, is_negative)
    }
}
