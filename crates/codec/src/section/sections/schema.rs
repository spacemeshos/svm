use svm_layout::{Id, Primitive, SymbolicVar, Type};
use svm_types::SchemaSection;

use crate::{Codec, ParseError, ReadExt, WriteExt};

impl Codec for SchemaSection {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        (self.vars().len() as u16).encode(w);

        for var in self.vars() {
            var.id().0.encode(w);
            var.ty().encode(w);
            var.name().to_string().encode(w);
        }
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let var_count = u16::decode(reader)?;
        let mut section = Self::with_capacity(var_count as usize);

        for _ in 0..var_count {
            let id = Id(u32::decode(reader)?);
            let ty = Type::decode(reader)?;
            let name = String::decode(reader)?;

            section.push_var(SymbolicVar::new(id, name, ty));
        }

        Ok(section)
    }
}

fn primitive_to_u8(prim: Primitive) -> u8 {
    match prim {
        Primitive::Address => 0,
        Primitive::Amount => 1,
        Primitive::Bool => 2,
        Primitive::I16 => 3,
        Primitive::I32 => 4,
        Primitive::I64 => 5,
        Primitive::I8 => 6,
        Primitive::U16 => 7,
        Primitive::U32 => 8,
        Primitive::U64 => 9,
        Primitive::U8 => 10,
    }
}

fn u8_to_primitive(val: u8) -> Option<Primitive> {
    Some(match val {
        0 => Primitive::Address,
        1 => Primitive::Amount,
        2 => Primitive::Bool,
        3 => Primitive::I16,
        4 => Primitive::I32,
        5 => Primitive::I64,
        6 => Primitive::I8,
        7 => Primitive::U16,
        8 => Primitive::U32,
        9 => Primitive::U64,
        10 => Primitive::U8,
        _ => return None,
    })
}

impl Codec for svm_layout::Type {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        match self {
            Type::Primitive(primitive) => {
                0xfu8.encode(w);
                primitive_to_u8(*primitive).encode(w);
            }
            Type::Array { primitive, length } => {
                assert!(*length < 0xf);
                (*length as u8).encode(w);
                primitive_to_u8(*primitive).encode(w);
            }
        }
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let bytes = <[u8; 2]>::decode(reader)?;
        let primitive = u8_to_primitive(bytes[1]).ok_or(ParseError::BadByte(bytes[1]))?;

        Ok(match bytes[0] {
            0xf => Type::Primitive(primitive),
            length => Type::Array {
                primitive,
                length: length as usize,
            },
        })
    }

    fn fixed_size() -> Option<usize> {
        Some(1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::codec::test_codec_bool;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    const MAX_LENGTH: usize = 0xf - 1;

    #[derive(Debug, Clone)]
    struct PrimitiveWrapper(Primitive);

    impl Arbitrary for PrimitiveWrapper {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let alternatives: Vec<u8> = (0..=10).collect();
            let byte: u8 = *g.choose(&alternatives).unwrap();
            Self(u8_to_primitive(byte).unwrap())
        }
    }

    #[derive(Debug, Clone)]
    struct TypeWrapper(Type);

    impl Arbitrary for TypeWrapper {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let array_length = <Option<usize>>::arbitrary(g);
            match array_length {
                None => Self(Type::Primitive(PrimitiveWrapper::arbitrary(g).0)),
                Some(len) => Self(Type::Array {
                    length: len.min(MAX_LENGTH),
                    primitive: PrimitiveWrapper::arbitrary(g).0,
                }),
            }
        }
    }

    #[quickcheck]
    fn schema_section(mut vars: Vec<(u32, String, TypeWrapper)>) -> bool {
        vars.truncate(0xf);
        let mut section = SchemaSection::new();

        for var in vars.into_iter() {
            section.push_var(SymbolicVar::new(Id(var.0), var.1, var.2 .0));
        }

        test_codec_bool(section)
    }
}
