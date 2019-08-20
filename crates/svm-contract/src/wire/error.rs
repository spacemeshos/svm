use super::field::Field;

pub enum Error {
    EmptyName,
    NameNotValidUTF8String,
    DepsNotSupportedYet,
    AdminsNotSupportedYet,
    NotEnoughBytes(Field),
    UnsupportedProtoVersion(u32),
    NoAuthors,
    InvalidWasm,
}
