use svm_layout::{Id, SymbolicVar, Type};
use svm_types::SchemaSection;

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Field, WriteExt};
use crate::{ParseError, ReadExt};

impl SectionEncoder for SchemaSection {
    fn encode(&self, w: &mut Vec<u8>) {
        encode_var_count(self, w);

        for var in self.vars() {
            encode_var(var, w);
        }
    }
}

impl SectionDecoder for SchemaSection {
    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, ParseError> {
        let var_count = decode_var_count(cursor)?;
        let mut section = Self::new();

        for _ in 0..var_count {
            section.push_var(decode_var(cursor)?);
        }

        Ok(section)
    }
}

fn encode_var(var: &SymbolicVar, w: &mut Vec<u8>) {
    encode_var_name(var, w);
    w.write_type_sig(var.ty().clone());
}

fn encode_var_count(schema: &SchemaSection, w: &mut Vec<u8>) {
    w.write_u16_be(schema.vars().len() as u16);
}

fn decode_var_count(cursor: &mut std::io::Cursor<&[u8]>) -> Result<u16, ParseError> {
    cursor
        .read_u16_be()
        .map_err(|_| ParseError::NotEnoughBytes(Field::SymbolicVarCount))
}

fn decode_var(cursor: &mut std::io::Cursor<&[u8]>) -> Result<SymbolicVar, ParseError> {
    let id = Id(0);
    let name = decode_var_name(cursor)?;
    let ty = decode_var_ty(cursor)?;

    let var = SymbolicVar::new(id, name, ty);

    Ok(var)
}

fn encode_var_name(var: &SymbolicVar, w: &mut Vec<u8>) {
    let name = var.name();

    w.write_string(name);
}

fn decode_var_name(cursor: &mut std::io::Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(name)) => Ok(name),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::SymbolicVarName)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::SymbolicVarName)),
    }
}

fn decode_var_ty(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Type, ParseError> {
    match cursor.read_type_sig() {
        Ok(Some(ty)) => Ok(ty),
        _ => Err(ParseError::NotEnoughBytes(Field::SymbolicVarName)),
    }
}
