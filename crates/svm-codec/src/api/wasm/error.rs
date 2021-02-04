use std::fmt;

use super::{to_wasm_buffer, wasm_buffer_data, BUF_ERROR_MARKER};

pub fn into_error_buffer<T: fmt::Debug>(err: T) -> usize {
    let msg: String = format!("{:?}", err);
    let bytes = msg.as_bytes();

    let mut buf = Vec::with_capacity(1 + bytes.len());

    buf.push(BUF_ERROR_MARKER);
    buf.extend_from_slice(bytes);

    to_wasm_buffer(&buf)
}

pub unsafe fn error_as_string(buf: usize) -> String {
    let bytes = wasm_buffer_data(buf);
    assert_eq!(bytes[0], BUF_ERROR_MARKER);

    // skipping the `ERROR` marker
    let bytes = bytes[1..].to_vec();

    String::from_utf8_unchecked(bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::api::wasm;

    #[derive(Debug)]
    struct MyError {
        reason: String,
    }

    #[test]
    fn wasm_into_error_buffer() {
        let err = MyError {
            reason: "An error has occurred...".to_string(),
        };

        let buf = into_error_buffer(err);

        let loaded = unsafe { error_as_string(buf) };
        println!("{:?}", loaded);
        assert_eq!(loaded, r#"MyError { reason: "An error has occurred..." }"#);

        wasm::free(buf);
    }
}
