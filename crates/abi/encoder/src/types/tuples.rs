use crate::{ByteSize, Encoder};

impl<T, W> Encoder<W> for (T,)
where
    T: Encoder<W>,
{
    fn encode(&self, w: &mut W) {
        self.0.encode(w);
    }
}

impl<T> ByteSize for (T,)
where
    T: ByteSize,
{
    fn byte_size(&self) -> usize {
        self.0.byte_size()
    }

    fn max_byte_size() -> usize {
        T::max_byte_size()
    }
}

impl<T0, T1, W> Encoder<W> for (T0, T1)
where
    T0: Encoder<W>,
    T1: Encoder<W>,
{
    fn encode(&self, w: &mut W) {
        self.0.encode(w);
        self.1.encode(w);
    }
}

impl<T0, T1> ByteSize for (T0, T1)
where
    T0: ByteSize,
    T1: ByteSize,
{
    fn byte_size(&self) -> usize {
        self.0.byte_size() + self.1.byte_size()
    }

    fn max_byte_size() -> usize {
        T0::max_byte_size() + T1::max_byte_size()
    }
}

impl<T0, T1, T2, W> Encoder<W> for (T0, T1, T2)
where
    T0: Encoder<W>,
    T1: Encoder<W>,
    T2: Encoder<W>,
{
    fn encode(&self, w: &mut W) {
        self.0.encode(w);
        self.1.encode(w);
        self.2.encode(w);
    }
}

impl<T0, T1, T2> ByteSize for (T0, T1, T2)
where
    T0: ByteSize,
    T1: ByteSize,
    T2: ByteSize,
{
    fn byte_size(&self) -> usize {
        self.0.byte_size() + self.1.byte_size() + self.2.byte_size()
    }

    fn max_byte_size() -> usize {
        T0::max_byte_size() + T1::max_byte_size() + T2::max_byte_size()
    }
}

impl<T0, T1, T2, T3, W> Encoder<W> for (T0, T1, T2, T3)
where
    T0: Encoder<W>,
    T1: Encoder<W>,
    T2: Encoder<W>,
    T3: Encoder<W>,
{
    fn encode(&self, w: &mut W) {
        self.0.encode(w);
        self.1.encode(w);
        self.2.encode(w);
        self.3.encode(w);
    }
}

impl<T0, T1, T2, T3> ByteSize for (T0, T1, T2, T3)
where
    T0: ByteSize,
    T1: ByteSize,
    T2: ByteSize,
    T3: ByteSize,
{
    fn byte_size(&self) -> usize {
        self.0.byte_size() + self.1.byte_size() + self.2.byte_size() + self.3.byte_size()
    }

    fn max_byte_size() -> usize {
        T0::max_byte_size() + T1::max_byte_size() + T2::max_byte_size() + T3::max_byte_size()
    }
}

impl<T0, T1, T2, T3, T4, W> Encoder<W> for (T0, T1, T2, T3, T4)
where
    T0: Encoder<W>,
    T1: Encoder<W>,
    T2: Encoder<W>,
    T3: Encoder<W>,
    T4: Encoder<W>,
{
    fn encode(&self, w: &mut W) {
        self.0.encode(w);
        self.1.encode(w);
        self.2.encode(w);
        self.3.encode(w);
        self.4.encode(w);
    }
}

impl<T0, T1, T2, T3, T4> ByteSize for (T0, T1, T2, T3, T4)
where
    T0: ByteSize,
    T1: ByteSize,
    T2: ByteSize,
    T3: ByteSize,
    T4: ByteSize,
{
    fn byte_size(&self) -> usize {
        self.0.byte_size()
            + self.1.byte_size()
            + self.2.byte_size()
            + self.3.byte_size()
            + self.4.byte_size()
    }

    fn max_byte_size() -> usize {
        T0::max_byte_size()
            + T1::max_byte_size()
            + T2::max_byte_size()
            + T3::max_byte_size()
            + T4::max_byte_size()
    }
}

impl<T0, T1, T2, T3, T4, T5, W> Encoder<W> for (T0, T1, T2, T3, T4, T5)
where
    T0: Encoder<W>,
    T1: Encoder<W>,
    T2: Encoder<W>,
    T3: Encoder<W>,
    T4: Encoder<W>,
    T5: Encoder<W>,
{
    fn encode(&self, w: &mut W) {
        self.0.encode(w);
        self.1.encode(w);
        self.2.encode(w);
        self.3.encode(w);
        self.4.encode(w);
        self.5.encode(w);
    }
}

impl<T0, T1, T2, T3, T4, T5> ByteSize for (T0, T1, T2, T3, T4, T5)
where
    T0: ByteSize,
    T1: ByteSize,
    T2: ByteSize,
    T3: ByteSize,
    T4: ByteSize,
    T5: ByteSize,
{
    fn byte_size(&self) -> usize {
        self.0.byte_size()
            + self.1.byte_size()
            + self.2.byte_size()
            + self.3.byte_size()
            + self.4.byte_size()
            + self.5.byte_size()
    }

    fn max_byte_size() -> usize {
        T0::max_byte_size()
            + T1::max_byte_size()
            + T2::max_byte_size()
            + T3::max_byte_size()
            + T4::max_byte_size()
            + T5::max_byte_size()
    }
}
