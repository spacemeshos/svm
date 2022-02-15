/// A marker trait for Primitives
pub trait PrimitiveMarker {}

macro_rules! mark_primitive {
    ($($ty:ty),*) => {
        $( impl PrimitiveMarker for $ty {} )*
    };
}

mark_primitive!(());
mark_primitive!(bool);
mark_primitive!(u8, i8);
mark_primitive!(u16, i16);
mark_primitive!(u32, i32);
mark_primitive!(u64, i64);
