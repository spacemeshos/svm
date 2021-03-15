/// A trait used to encoding a value (of `Primitive` or `Composite` type)

pub trait Encoder<W> {
    /// Encodes `self` and outputs the data into `w`
    fn encode(&self, w: &mut W);
}

impl<T, W> Encoder<W> for &T
where
    T: Encoder<W>,
{
    fn encode(&self, w: &mut W) {
        (**self).encode(w);
    }
}

impl<T, W> Encoder<W> for &mut T
where
    T: Encoder<W>,
{
    fn encode(&self, w: &mut W) {
        (**self).encode(w);
    }
}

pub trait ByteSize {
    fn byte_size(&self) -> usize;

    fn max_byte_size() -> usize;
}
