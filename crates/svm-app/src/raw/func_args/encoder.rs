use crate::nib;

use crate::types::{WasmType, WasmValue};

use super::super::{Field, Nibble, NibbleWriter};
use super::WasmValueLayout;

const NO_MORE: u8 = 0b_0110;

pub fn encode_func_args(args: &[WasmValue], writer: &mut NibbleWriter) {
    let mut layouts = Vec::with_capacity(args.len());

    for arg in args.iter() {
        let layout = func_arg_layout(arg);
        let nib = (&layout).into();

        layouts.push(layout);
        writer.write(&[nib]);
    }

    // output `no more func args layouts` marker.
    let no_more_nib = nib!(NO_MORE);
    writer.write(&[no_more_nib]);

    // write the args values
    for (i, arg) in args.iter().enumerate() {
        let layout = &layouts[i];

        encode_func_arg(arg, layout, writer);
    }
}

fn func_arg_layout(arg: &WasmValue) -> WasmValueLayout {
    match arg {
        WasmValue::I32(v) => {
            let len = func_arg_byte_length(*v as u64);
            debug_assert!(len <= 4);

            WasmValueLayout {
                ty: WasmType::I32,
                len,
            }
        }
        WasmValue::I64(v) => {
            let len = func_arg_byte_length(*v);
            debug_assert!(len <= 8);

            WasmValueLayout {
                ty: WasmType::I64,
                len,
            }
        }
    }
}

fn func_arg_byte_length(value: u64) -> usize {
    match value {
        0 => 0,
        0x01..=0xFF => 1,
        0x01_00..=0xFF_FF => 2,
        0x_01_00_00..=0xFF_FF_FF => 3,
        0x_01_00_00_00..=0xFF_FF_FF_FF => 4,
        0x_01_00_00_00_00..=0xFF_FF_FF_FF_FF => 5,
        0x_01_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF => 6,
        0x_01_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF => 7,
        0x_01_00_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF_FF => 8,
        _ => unreachable!(),
    }
}

fn encode_func_arg(arg: &WasmValue, layout: &WasmValueLayout, writer: &mut NibbleWriter) {
    let mut nibbles = Vec::with_capacity(layout.len);

    let mut val = match arg {
        WasmValue::I32(v) => *v as u64,
        WasmValue::I64(v) => *v,
    };

    for _ in 0..layout.len {
        let byte = (val & 0x0F) as u8;
        let nib = nib!(byte);

        nibbles.push(nib);

        val >>= 4;
    }

    // since we've scanned `val` from `lsb` to `msb` order,
    // we need to reverse `nibbles` prior calling `writer` with them.
    let nibbles: Vec<_> = nibbles.drain(..).rev().collect();

    writer.write(&nibbles[..])
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode_func_args_zero_args() {
        todo!()
    }
}
