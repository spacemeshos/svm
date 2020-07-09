use std::fmt::Write;

/// Converts `bytes` into a `String` represented in hex base.
pub fn fmt_hex(bytes: &[u8], separator: &str) -> String {
    let mut buf = String::with_capacity(bytes.len() * 3);

    let n = bytes.len();

    if n > 0 {
        for b in bytes.iter().take(n - 1) {
            let _ = buf.write_fmt(format_args!("{:02X}{}", b, separator));
        }

        // last byte has no following separator
        let last = bytes.last().unwrap();
        let _ = buf.write_fmt(format_args!("{:02X}", last));
    }

    buf
}

#[cfg(test)]
mod tests {
    #[test]
    fn fmt_hex() {
        let bytes = vec![0x01, 0x20, 0x30, 0x40];

        assert_eq!("01 20 30 40", crate::fmt::fmt_hex(bytes.as_slice(), " "));
        assert_eq!("01,20,30,40", crate::fmt::fmt_hex(bytes.as_slice(), ","));
        assert_eq!(
            "01, 20, 30, 40",
            crate::fmt::fmt_hex(bytes.as_slice(), ", ")
        );
    }
}
