use crate::raw::{helpers, NibbleWriter};

pub fn encode_deploy_template(version: u32, name: &str, page_count: u16, code: &[u8]) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    write_version(version, &mut w);
    write_name(name, &mut w);
    write_page_count(page_count, &mut w);
    write_code(code, &mut w);

    helpers::bytes(&mut w)
}

fn write_version(version: u32, w: &mut NibbleWriter) {
    helpers::encode_version(version, w);
}

fn write_name(name: &str, w: &mut NibbleWriter) {
    helpers::encode_string(name, w);
}

fn write_page_count(page_count: u16, w: &mut NibbleWriter) {
    helpers::encode_varuint14(page_count, w);
}

fn write_code(code: &[u8], w: &mut NibbleWriter) {
    w.write_bytes(code)
}
