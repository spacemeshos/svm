use crate::traits::Push;
use crate::{ByteSize, Encoder};

impl<T, W> Encoder<W> for &[T]
where
    T: Encoder<W>,
    W: Push<Item = u8>,
{
    fn encode(&self, w: &mut W) {
        assert!(self.len() < 11);

        w.push(layout_array(self.len()));

        seq_macro::seq!(i in 0..11 {
            if self.len() > i {
                self[i].encode(w);
            }
        });
    }
}

impl<T, W, const N: usize> Encoder<W> for [T; N]
where
    T: Encoder<W>,
    W: Push<Item = u8>,
{
    #[inline]
    fn encode(&self, w: &mut W) {
        (&self[..]).encode(w)
    }
}

impl<T, const N: usize> ByteSize for [T; N]
where
    T: ByteSize,
{
    fn byte_size(&self) -> usize {
        assert!(N < 11);

        let mut payload_size = 0;
        seq_macro::seq!(i in 0..11 {
            // The compiler complains, but it's wrong! The following comparison
            // might be useless or not, depending on the array const generic.
            #[allow(unused_comparisons)]
            if N >= i {
                payload_size += self[i].byte_size();
            }
        });
        1 + payload_size
    }

    fn max_byte_size() -> usize {
        1 + T::max_byte_size() * N
    }
}

/// Calculates the layout marker byte of an array of size `len`.
const fn layout_array(len: usize) -> u8 {
    if len < 8 {
        0b_0_000_0110 | (len << 4) as u8
    } else {
        0b_0_000_0111 | ((len - 8) << 4) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_empty() {
        assert_eq!(<[bool; 0]>::max_byte_size(), 1);
    }

    #[test]
    fn array_one_item() {
        assert_eq!(<[bool; 1]>::max_byte_size(), 1 + bool::max_byte_size());
        assert_eq!(<[u32; 1]>::max_byte_size(), 1 + u32::max_byte_size());
    }

    #[test]
    fn array_two_items() {
        assert_eq!(<[bool; 2]>::max_byte_size(), 1 + bool::max_byte_size() * 2);
        assert_eq!(<[u32; 2]>::max_byte_size(), 1 + u32::max_byte_size() * 2);
    }
}
