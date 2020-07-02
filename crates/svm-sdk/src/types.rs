pub enum Primitive {
    Blob,

    Address,

    PubKey256,
}

pub enum Composite<'a> {
    Array(&'a [Type<'a>]),
}

pub enum Type<'a> {
    Primitive(Primitive),
    Composite(Composite<'a>),
}
