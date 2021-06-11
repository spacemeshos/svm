use std::io::Cursor;
use std::iter::Scan;

use svm_layout::{Id, Primitive, SymbolicVar, Type};
use svm_types::{SchemaSection, SectionKind};

use crate::{r#type, section};
use crate::{Field, ParseError, ReadExt, WriteExt};

use super::SectionEncoder;

impl SectionEncoder for SchemaSection {
    fn encode(&self, w: &mut Vec<u8>) {
        // let mut raw_section = Vec::new();

        // encode_var_count(self, &mut raw_section);

        // for var in self.vars() {
        //     encode_var(var, &mut raw_section);
        // }

        // let section = SectionHeader {
        //     kind: SectionKind::Schema,
        //     byte_size: raw_section.len() as u32,
        // };

        // section::encode(&section, w);

        // w.write_bytes(&raw_section);
    }

    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, ParseError> {
        let var_count = decode_var_count(cursor)? as usize;

        let mut vars = Vec::with_capacity(var_count);

        for _ in 0..var_count {
            let var = decode_var(cursor)?;

            vars.push(var);
        }

        let section = SchemaSection { vars };

        Ok(section)
    }
}

fn encode_var(var: &SymbolicVar, w: &mut Vec<u8>) {
    encode_var_name(var, w);

    r#type::encode_type(var.ty(), w);
}

fn encode_var_count(schema: &SchemaSection, w: &mut Vec<u8>) {
    w.write_u16_be(schema.vars().len() as u16);
}

fn decode_var_count(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    cursor
        .read_u16_be()
        .map_err(|_| ParseError::NotEnoughBytes(Field::SymbolicVarCount))
}

fn decode_var(cursor: &mut Cursor<&[u8]>) -> Result<SymbolicVar, ParseError> {
    todo!("encode var id...");

    let id = Id(0);
    let name = decode_var_name(cursor)?;
    let ty = r#type::decode_type(cursor)?;

    let var = SymbolicVar::new(id, name, ty);

    Ok(var)
}

fn encode_var_name(var: &SymbolicVar, w: &mut Vec<u8>) {
    let name = var.name();

    w.write_string(name);
}

fn decode_var_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(name)) => Ok(name),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::SymbolicVarName)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::SymbolicVarName)),
    }
}
