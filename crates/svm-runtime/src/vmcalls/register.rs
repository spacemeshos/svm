use crate::helpers;

pub fn reg_replace_byte(
    ctx: &mut wasmer_runtime::Ctx,
    reg_bits: i32,
    reg_idx: i32,
    byte: i32,
    offset: i32,
) {
    log::debug!(
        "replace_byte register=`{}:{}`, byte={}, offset={}",
        reg_bits,
        reg_idx,
        byte,
        offset
    );

    let byte = byte as u32;
    assert!(byte <= 0xFF);

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.replace_byte(byte as u8, offset);
}

pub fn reg_read_be_i64(ctx: &mut wasmer_runtime::Ctx, reg_bits: i32, reg_idx: i32) -> i64 {
    use byteorder::{BigEndian, ByteOrder};

    log::debug!("`reg_read_be_i64` register=`{}:{}`", reg_bits, reg_idx);

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    let buf = reg.getn(8);

    BigEndian::read_i64(&buf)
}

pub fn reg_write_be_i64(ctx: &mut wasmer_runtime::Ctx, value: i64, reg_bits: i32, reg_idx: i32) {
    use byteorder::{BigEndian, ByteOrder};

    log::debug!(
        "`reg_write_be_i64` register=`{}:{}`, value={}",
        reg_bits,
        reg_idx,
        value
    );

    let mut buf = [0; 8];
    BigEndian::write_i64(&mut buf, value);

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(&buf);
}
