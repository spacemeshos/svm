use crate::{DecDigit, String, StringBuilder};

/// A trait to be implemented by types that want to have a [`String`] representation.
pub trait ToString {
    /// Returns a [`String`] representing `self`.
    fn to_string(&self) -> String;
}

#[inline(never)]
fn num_as_string(num: u64, is_negative: bool) -> String {
    // Important: we allocate 20 digits in order to be able to hold [`std::u64::MAX`].
    let mut value = num;
    let mut digits = [0u8; 20];
    let mut used_count = 0;

    seq_macro::seq!(N in 0..21 {
        let (digit, new_value, completed) = next_digit(value);
        digits[used_count] = digit;

        used_count += 1;
        value = new_value;

        if completed {
            debug_assert_eq!(value, 0);
            return concat_digits(&digits, used_count, is_negative);
        }
    });

    // we should never get here
    crate::panic()
}

#[inline]
fn next_digit(value: u64) -> (u8, u64, bool) {
    let digit = value % 10;
    let value = value / 10;
    let completed = value == 0;

    debug_assert!(digit < 10);
    (digit as u8, value, completed)
}

#[inline(never)]
fn concat_digits(digits: &[u8; 20], used_count: usize, is_negative: bool) -> String {
    // We allocate 21 digits and not 20 for the `minus` sign.
    let mut sb = StringBuilder::with_capacity(21);
    if is_negative {
        sb.push_str(&String::from_byte(b'-'));
    }

    seq_macro::seq!(N in 0..20 {
        if N < used_count {
            let digit = digits[used_count - N - 1];
            let digit_str = DecDigit(digit).to_string();
            sb.push_str(&digit_str);
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
        let num = (*self as i64).abs();
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
        let num = (*self as i64).abs();
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
        let num = (*self as i64).abs();
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
        let num = (*self as i128).abs();
        let is_negative = *self < 0;
        num_as_string(num as u64, is_negative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;

    fn test<T: ToString>(value: T, expected: &'static str) {
        assert_eq!(
            value.to_string().to_std_string(),
            std::string::ToString::to_string(expected)
        );
    }

    #[test]
    fn u8_to_string() {
        test(0u8, "0");
        test(12u8, "12");
        test(123u8, "123");
        test(std::u8::MAX, "255");
    }

    #[test]
    fn i8_to_string() {
        test(0i8, "0");
        test(-0i8, "0");

        test(7i8, "7");
        test(-7i8, "-7");

        test(12i8, "12");
        test(-12i8, "-12");

        test(123i8, "123");
        test(-123i8, "-123");

        test(std::i8::MAX, "127");
        test(std::i8::MIN, "-128");
    }

    #[test]
    fn u16_to_string() {
        test(0u16, "0");
        test(12u16, "12");
        test(123u16, "123");
        test(std::u16::MAX, "65535");
    }

    #[test]
    fn i16_to_string() {
        test(0i16, "0");
        test(-0i16, "0");

        test(7i16, "7");
        test(-7i16, "-7");

        test(12i16, "12");
        test(-12i16, "-12");

        test(123i16, "123");
        test(-123i16, "-123");

        test(std::i16::MAX, "32767");
        test(std::i16::MIN, "-32768");
    }

    #[test]
    fn u32_to_string() {
        test(0u32, "0");
        test(12u32, "12");
        test(123u32, "123");
        test(std::u32::MAX, "4294967295");
    }

    #[test]
    fn i32_to_string() {
        test(0i32, "0");
        test(-0i32, "0");

        test(7i32, "7");
        test(-7i32, "-7");

        test(12i32, "12");
        test(-12i32, "-12");

        test(123i32, "123");
        test(-123i32, "-123");

        test(std::i32::MAX, "2147483647");
        test(std::i32::MIN, "-2147483648");
    }

    #[test]
    fn u64_to_string() {
        test(0u64, "0");
        test(12u64, "12");
        test(123u64, "123");
        test(std::u64::MAX, "18446744073709551615");
    }

    #[test]
    fn i64_to_string() {
        test(0i64, "0");
        test(-0i64, "0");

        test(7i64, "7");
        test(-7i64, "-7");

        test(12i64, "12");
        test(-12i64, "-12");

        test(123i64, "123");
        test(-123i64, "-123");

        test(std::i64::MAX, "9223372036854775807");
        test(std::i64::MIN, "-9223372036854775808");
    }
}
