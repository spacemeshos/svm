use std::convert::{TryFrom, TryInto};
use std::io::{self, ErrorKind};

use byteorder::{BigEndian, ByteOrder, ReadBytesExt};

use crate::svm_byte_array;

pub use svm_layout::DataLayout;

///
/// Parsing raw `DataLayout` given as `svm_byte_array` into `DataLayout`.
///
/// # Example
///
/// ```
/// use std::io;
/// use std::convert::TryFrom;
///
/// use svm_layout::{VarId, DataLayout};
/// use svm_runtime_c_api::svm_byte_array;
///
/// let data: Vec<u8> = vec![0, 0, 0, 10, 0, 0, 0, 20, 0, 0, 0, 30];
/// let bytes: svm_byte_array = data.into();
///
/// let layout: Result<DataLayout, io::Error> = DataLayout::try_from(bytes);
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
impl TryFrom<&svm_byte_array> for DataLayout {
    type Error = io::Error;

    fn try_from(bytes: &svm_byte_array) -> Result<Self, Self::Error> {
        if bytes.length % 4 != 0 {
            return Err(ErrorKind::InvalidInput.into());
        }

        let bytes: &[u8] = bytes.into();
        let raw_layout: Vec<u32> = bytes.chunks(4).map(BigEndian::read_u32).collect();

        let layout = DataLayout::from(&raw_layout[..]);
        Ok(layout)
    }
}

impl TryFrom<svm_byte_array> for DataLayout {
    type Error = io::Error;

    #[inline]
    fn try_from(bytes: svm_byte_array) -> Result<Self, Self::Error> {
        (&bytes).try_into()
    }
}
