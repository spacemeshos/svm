mod decoder;
mod encoder;
mod layout;

pub use decoder::decode_func_args;
pub use encoder::encode_func_args;
pub use layout::{
    WasmValueLayout, DO_SKIP, I32_0B, I32_1B, I32_2B, I32_3B, I32_4B, I64_0B, I64_1B, I64_2B,
    I64_3B, I64_4B, I64_5B, I64_6B, I64_7B, I64_8B, NO_MORE,
};

#[cfg(test)]
mod tests {
    use crate::{
        nib,
        types::{WasmType, WasmValue},
    };

    use super::super::{NibbleIter, NibbleWriter};
    use super::{decode_func_args, encode_func_args, WasmValueLayout, DO_SKIP};

    fn assert_encode_decode(args: Vec<WasmValue>) {
        let mut writer = NibbleWriter::new();

        // each func arg layout takes exactly one nibble
        // plus there is one nibble for `no more func args marker`
        let layouts_nibble_count = args.len() + 1;

        if layouts_nibble_count % 2 == 1 {
            let skip_nib = nib!(DO_SKIP);
            writer.write(&[skip_nib]);
        }

        encode_func_args(&args[..], &mut writer);

        let data = writer.bytes();
        let mut iter = NibbleIter::new(&data);

        let decoded = decode_func_args(&mut iter).unwrap();
        assert_eq!(args, decoded);
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
