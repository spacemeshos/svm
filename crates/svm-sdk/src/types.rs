pub mod marker {
    pub const ARRAY_START: u8 = 0;

    pub const ARRAY_END: u8 = 1;

    pub const ADDRESS: u8 = 2;

    pub const PUBKEY_256: u8 = 3;
}

pub enum Primitive {
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
