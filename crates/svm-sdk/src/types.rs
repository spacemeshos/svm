use core::convert::TryFrom;

pub enum Primitive {
    Blob1,

    Blob2,

    Blob3,

    Address,

    PubKey256,
}

pub enum Composite<'a> {
    Array(&'a [Type<'a>]),
    Tuple(&'a [Type<'a>]),
}

pub enum Type<'a> {
    Primitive(Primitive),
    Composite(Composite<'a>),
}

impl From<Primitive> for u8 {
    fn from(ty: Primitive) -> u8 {
        match ty {
            Primitive::Blob1 => 2,
            Primitive::Blob2 => 3,
            Primitive::Blob3 => 4,
            Primitive::Address => 5,
            Primitive::PubKey256 => 6,
        }
    }
}

impl From<Composite<'_>> for u8 {
    fn from(ty: Composite) -> u8 {
        match ty {
            Composite::Array(..) => 0,
            Composite::Tuple(..) => 1,
        }
    }
}

impl From<Type<'_>> for u8 {
    fn from(ty: Type) -> u8 {
        match ty {
            Type::Composite(ty) => ty.into(),
            Type::Primitive(ty) => ty.into(),
        }
    }
}
