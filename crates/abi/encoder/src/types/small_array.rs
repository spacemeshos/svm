use svm_abi_layout::layout;
use svm_sdk_std::Vec;

use crate::{ByteSize, Encoder};

// TODO:
// for a detailed explanation on how to make the following code
// more ergonomic see look at `address.rs` under this module.
// There is also an issue for that: [Issue #230](https://github.com/spacemeshos/svm/issues/230)

macro_rules! impl_encode {
    ($W:ty) => {
        impl<T: Encoder<$W>> Encoder<$W> for &[T] {
            fn encode(&self, w: &mut $W) {
                assert!(self.len() < 11);

                let marker = match self.len() {
                    0 => layout::ARR_0,
                    1 => layout::ARR_1,
                    2 => layout::ARR_2,
                    3 => layout::ARR_3,
                    4 => layout::ARR_4,
                    5 => layout::ARR_5,
                    6 => layout::ARR_6,
                    7 => layout::ARR_7,
                    8 => layout::ARR_8,
                    9 => layout::ARR_9,
                    10 => layout::ARR_10,
                    _ => svm_sdk_std::panic(),
                };

                w.push(marker);

                let mut iter = self.iter();

                match self.len() {
                    0 => impl_encode!(0 iter w),
                    1 => impl_encode!(1 iter w),
                    2 => impl_encode!(2 iter w),
                    3 => impl_encode!(3 iter w),
                    4 => impl_encode!(4 iter w),
                    5 => impl_encode!(5 iter w),
                    6 => impl_encode!(6 iter w),
                    7 => impl_encode!(7 iter w),
                    8 => impl_encode!(8 iter w),
                    9 => impl_encode!(9 iter w),
                    10 => impl_encode!(10 iter w),
                    _ => svm_sdk_std::panic(),
                };
            }
        }
    };

    (0 $iter:ident $w:ident) => {{  }};
    (1 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(0 $iter $w);
    }};
    (2 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(1 $iter $w);
    }};
    (3 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(2 $iter $w);
    }};
    (4 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(3 $iter $w);
    }};
    (5 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(4 $iter $w);
    }};
    (6 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(5 $iter $w);
    }};
    (7 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(6 $iter $w);
    }};
    (8 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(7 $iter $w);
    }};
    (9 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(8 $iter $w);
    }};
    (10 $iter:ident $w:ident) => {{
        impl_encode!(@ $iter $w);
        impl_encode!(9 $iter $w);
     }};
    (@ $iter:ident $w:ident) => {{
        let item = $iter.next().unwrap();

        item.encode($w);
    }}
}

impl_encode!(svm_sdk_std::Vec<u8>);

macro_rules! impl_array_encode {
    ($W:ty => $($N:expr),*) => {
        $( impl_array_encode!{@ $W => $N} )*
    };

    (@ $W:ty => $N:expr) => {
        impl<T: Encoder<$W>> Encoder<$W> for [T; $N] {
            #[inline]
            fn encode(&self, w: &mut $W) {
                (&self[..]).encode(w)
            }
        }
    };
}

impl_array_encode!(svm_sdk_std::Vec<u8> => 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

impl<T, const N: usize> ByteSize for [T; N]
where
    T: ByteSize,
{
    fn byte_size(&self) -> usize {
        assert!(N < 11);

        1 + match N {
            0 => 0,
            1 => self[0].byte_size(),
            2 => self[0].byte_size() + self[1].byte_size(),
            3 => self[0].byte_size() + self[1].byte_size() + self[2].byte_size(),
            4 => {
                self[0].byte_size()
                    + self[1].byte_size()
                    + self[2].byte_size()
                    + self[3].byte_size()
            }
            5 => {
                self[0].byte_size()
                    + self[1].byte_size()
                    + self[2].byte_size()
                    + self[3].byte_size()
                    + self[4].byte_size()
            }
            6 => {
                self[0].byte_size()
                    + self[1].byte_size()
                    + self[2].byte_size()
                    + self[3].byte_size()
                    + self[4].byte_size()
                    + self[5].byte_size()
            }
            7 => {
                self[0].byte_size()
                    + self[1].byte_size()
                    + self[2].byte_size()
                    + self[3].byte_size()
                    + self[4].byte_size()
                    + self[5].byte_size()
                    + self[6].byte_size()
            }
            8 => {
                self[0].byte_size()
                    + self[1].byte_size()
                    + self[2].byte_size()
                    + self[3].byte_size()
                    + self[4].byte_size()
                    + self[5].byte_size()
                    + self[6].byte_size()
                    + self[7].byte_size()
            }
            9 => {
                self[0].byte_size()
                    + self[1].byte_size()
                    + self[2].byte_size()
                    + self[3].byte_size()
                    + self[4].byte_size()
                    + self[5].byte_size()
                    + self[6].byte_size()
                    + self[7].byte_size()
                    + self[8].byte_size()
            }
            10 => {
                self[0].byte_size()
                    + self[1].byte_size()
                    + self[2].byte_size()
                    + self[3].byte_size()
                    + self[4].byte_size()
                    + self[5].byte_size()
                    + self[6].byte_size()
                    + self[7].byte_size()
                    + self[8].byte_size()
                    + self[9].byte_size()
            }
            _ => svm_sdk_std::panic(),
        }
    }

    fn max_byte_size() -> usize {
        1 + T::max_byte_size() * N
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
