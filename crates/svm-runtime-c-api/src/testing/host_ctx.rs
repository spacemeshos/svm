use byteorder::{BigEndian, WriteBytesExt};

pub fn write_version(buf: &mut Vec<u8>, version: u32) {
    buf.write_u32::<BigEndian>(version).unwrap();
}

pub fn write_field_count(buf: &mut Vec<u8>, field_count: u16) {
    buf.write_u16::<BigEndian>(field_count).unwrap();
}

pub fn write_field(buf: &mut Vec<u8>, index: u16, value: Vec<u8>) {
    buf.write_u16::<BigEndian>(index).unwrap();
    buf.write_u16::<BigEndian>(value.len() as u16).unwrap();
    buf.extend_from_slice(&value[..]);
}
