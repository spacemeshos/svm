use std::io::Cursor;

use svm_layout::{Primitive, Type};

use crate::ParseError;

pub fn encode_type(ty: &Type, w: &mut Vec<u8>) {
    match ty {
        Type::Primitive(ty) => encode_primitive(ty, w),
        Type::Array { .. } => {
            todo!()
        }
    }
}

pub fn decode_type(_cursor: &mut Cursor<&[u8]>) -> Result<Type, ParseError> {
    todo!()
}

fn encode_primitive(ty: &Primitive, _w: &mut Vec<u8>) {
    match ty {
        Primitive::Bool => (),
        _ => todo!(),
    }
}
pub fn decode_primitive(_cursor: &mut Cursor<&[u8]>) -> Result<Primitive, ParseError> {
    todo!()
}
