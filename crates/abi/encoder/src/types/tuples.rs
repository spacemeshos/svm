extern crate alloc;

use alloc::vec::Vec;

use crate::Encoder;

impl<T> Encoder for (T,)
where
    T: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        self.0.encode(w);
    }
}

impl<T0, T1> Encoder for (T0, T1)
where
    T0: Encoder,
    T1: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        self.0.encode(w);
        self.1.encode(w);
    }
}

impl<T0, T1, T2> Encoder for (T0, T1, T2)
where
    T0: Encoder,
    T1: Encoder,
    T2: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        self.0.encode(w);
        self.1.encode(w);
        self.2.encode(w);
    }
}

impl<T0, T1, T2, T3> Encoder for (T0, T1, T2, T3)
where
    T0: Encoder,
    T1: Encoder,
    T2: Encoder,
    T3: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        self.0.encode(w);
        self.1.encode(w);
        self.2.encode(w);
        self.3.encode(w);
    }
}

impl<T0, T1, T2, T3, T4> Encoder for (T0, T1, T2, T3, T4)
where
    T0: Encoder,
    T1: Encoder,
    T2: Encoder,
    T3: Encoder,
    T4: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        self.0.encode(w);
        self.1.encode(w);
        self.2.encode(w);
        self.3.encode(w);
        self.4.encode(w);
    }
}

impl<T0, T1, T2, T3, T4, T5> Encoder for (T0, T1, T2, T3, T4, T5)
where
    T0: Encoder,
    T1: Encoder,
    T2: Encoder,
    T3: Encoder,
    T4: Encoder,
    T5: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        self.0.encode(w);
        self.1.encode(w);
        self.2.encode(w);
        self.3.encode(w);
        self.4.encode(w);
        self.5.encode(w);
    }
}
