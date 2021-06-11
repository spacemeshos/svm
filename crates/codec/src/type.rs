use std::io::Cursor;

use svm_layout::{Primitive, Type};

use crate::ParseError;

pub fn encode_type(ty: &Type, w: &mut Vec<u8>) {
    match ty {
        Type::Primitive(ty) => encode_primitive(ty, w),
        Type::Array {
            primitive: ty,
            length,
        } => {
            todo!()
        }
    }
}

pub fn decode_type(cursor: &mut Cursor<&[u8]>) -> Result<Type, ParseError> {
    todo!()
}

fn encode_primitive(ty: &Primitive, w: &mut Vec<u8>) {
    match ty {
        Primitive::Bool => (),
        _ => todo!(),
    }
}
pub fn decode_primitive(cursor: &mut Cursor<&[u8]>) -> Result<Primitive, ParseError> {
    todo!()
}
