pub mod marker {
    pub const ARRAY_START: u8 = 0;

    pub const ARRAY_END: u8 = 1;

    pub const TUPLE_START: u8 = 2;

    pub const TUPLE_END: u8 = 3;

    pub const BLOB_1: u8 = 4;

    pub const BLOB_2: u8 = 5;

    pub const BLOB_3: u8 = 6;

    pub const ADDRESS: u8 = 7;

    pub const PUBKEY_256: u8 = 8;
}

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
