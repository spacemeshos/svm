use std::convert::{TryFrom, TryInto};
use std::io::{self, ErrorKind};

use byteorder::{BigEndian, ByteOrder};

use crate::svm_byte_array;

pub use svm_layout::FixedLayout;

///
/// Parsing a raw `layout` given as `svm_byte_array` into `svm_layout::Layout`.
///
/// Here is the raw representation of a `Layout`:
///
/// +-----------------------------------------------------------+
/// | var #0 length (4 bytes) | . . . | var #N length (4 bytes) |
/// +-----------------------------------------------------------+
///
/// Each variable length conumes exactly 4 bytes encoded in a Big-Endian order.
/// Given that, the `length` of the `svm_byte_array` must be divisble by 4.
///
/// # Example
///
/// ```
/// use std::io;
/// use std::convert::TryFrom;
///
/// use svm_layout::{VarId, Layout};
/// use svm_types::Type;
/// use svm_ffi::svm_byte_array;
///
/// let ty = Type::of::<Vec<u8>>();
/// let data: Vec<u8> = vec![0, 0, 0, 10, 0, 0, 0, 20, 0, 0, 0, 30];
/// let bytes: svm_byte_array = (ty, data).into();
///
/// let layout: Result<Layout, io::Error> = Layout::try_from(bytes);
/// assert!(layout.is_ok());
///
/// let layout = layout.unwrap();
///
/// assert_eq!(layout.len(), 3);
/// assert_eq!(layout.get_var(VarId(0)), (0,  10));
/// assert_eq!(layout.get_var(VarId(1)), (10, 20));
/// assert_eq!(layout.get_var(VarId(2)), (30, 30));
/// ```
///
impl TryFrom<&svm_byte_array> for FixedLayout {
    type Error = io::Error;

    fn try_from(bytes: &svm_byte_array) -> Result<Self, Self::Error> {
        if bytes.length % 4 != 0 {
            return Err(ErrorKind::InvalidInput.into());
        }

        let bytes: &[u8] = bytes.into();
        let raw_layout: Vec<u32> = bytes.chunks(4).map(BigEndian::read_u32).collect();

        let layout = FixedLayout::from(&raw_layout[..]);
        Ok(layout)
    }
}

impl TryFrom<svm_byte_array> for FixedLayout {
    type Error = io::Error;

    #[inline]
    fn try_from(bytes: svm_byte_array) -> Result<Self, Self::Error> {
        (&bytes).try_into()
    }
}
