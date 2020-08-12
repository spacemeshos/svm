mod decoder;
mod encoder;

pub use decoder::{decode_func_args, decode_func_rets};
pub use encoder::{encode_func_args, encode_func_rets};

#[cfg(test)]
mod tests {
    use svm_nibble::{NibbleIter, NibbleWriter};
    use svm_types::WasmValue;

    use crate::api::raw::{decode_func_args, encode_func_args};
    use crate::error::ParseError;

    fn assert_encode_decode(args: Vec<WasmValue>) {
        let mut w = NibbleWriter::new();

        encode_func_args(&args[..], &mut w);

        let data = w.into_bytes();
        let mut iter = NibbleIter::new(&data[..]);

        let decoded = decode_func_args(&mut iter).unwrap();
        assert_eq!(args, decoded);

        assert!(iter.ensure_eof(ParseError::ExpectedEOF).is_ok());
    }

    #[test]
    fn encode_decode_func_args_zero_args() {
        let args = vec![];
        assert_encode_decode(args);
    }

    #[test]
    fn encode_decode_func_multiple_i32_args() {
        let arg1 = WasmValue::I32(0);
        let arg2 = WasmValue::I32(std::u32::MAX.into());
        let arg3 = WasmValue::I32(std::u16::MAX.into());
        let arg4 = WasmValue::I32(std::u8::MAX.into());

        let args = vec![arg1, arg2, arg3, arg4];
        assert_encode_decode(args);
    }

    #[test]
    fn encode_decode_func_multiple_i64_args() {
        let arg1 = WasmValue::I64(0);
        let arg2 = WasmValue::I64(std::u64::MAX.into());
        let arg3 = WasmValue::I64(std::u32::MAX.into());
        let arg4 = WasmValue::I64(std::u16::MAX.into());

        let args = vec![arg1, arg2, arg3, arg4];
        assert_encode_decode(args);
    }

    #[test]
    fn encode_decode_func_multiple_i32_and_i64_args() {
        let arg1 = WasmValue::I32(std::u32::MAX.into());
        let arg2 = WasmValue::I64(0);
        let arg3 = WasmValue::I32(std::u16::MAX.into());
        let arg4 = WasmValue::I64(std::u64::MAX.into());
        let arg5 = WasmValue::I32(std::u8::MAX.into());
        let arg6 = WasmValue::I64(std::u32::MAX.into());

        let args = vec![arg1, arg2, arg3, arg4, arg5, arg6];
        assert_encode_decode(args);
    }
}
