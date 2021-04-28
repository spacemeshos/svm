macro_rules! impl_blob_type {
    ($ty:ident, $nbytes:expr) => {
        use core::char;
        use core::cmp::{Eq, PartialEq};

        use svm_sdk_std::{ensure, Vec};

        #[allow(missing_docs)]
        #[repr(transparent)]
        #[derive(Copy, Hash)]
        pub struct $ty(*const u8);

        impl $crate::types::PrimitiveMarker for $ty {}

        impl core::clone::Clone for $ty {
            fn clone(&self) -> Self {
                $ty(self.0)
            }
        }

        impl $ty {
            #[allow(missing_docs)]
            #[inline]
            pub fn offset(&self) -> usize {
                self.as_ptr() as _
            }

            #[allow(missing_docs)]
            #[inline]
            pub fn as_ptr(&self) -> *const u8 {
                self.0
            }

            #[allow(missing_docs)]
            #[inline]
            pub const fn len() -> usize {
                $nbytes
            }

            #[allow(missing_docs)]
            #[inline]
            pub fn as_slice(&self) -> &[u8] {
                unsafe { core::slice::from_raw_parts(self.0, Self::len()) }
            }
        }

        impl From<*const u8> for $ty {
            #[inline]
            fn from(ptr: *const u8) -> Self {
                $ty(ptr)
            }
        }

        impl From<u32> for $ty {
            #[inline]
            fn from(offset: u32) -> Self {
                let ptr = offset as *const u8;

                ptr.into()
            }
        }

        impl From<&'static [u8]> for $ty {
            #[inline]
            fn from(bytes: &'static [u8]) -> Self {
                ensure!(bytes.len() == $nbytes);

                $ty(bytes.as_ptr())
            }
        }

        impl From<Vec<u8>> for $ty {
            #[inline]
            fn from(vec: Vec<u8>) -> Self {
                // TODO: leak `vec`

                ensure!(vec.len() == Self::len());

                let slice = vec.leak();
                let ptr = slice.as_ptr();

                $ty(ptr)
            }
        }

        impl $ty {
            pub fn repeat(byte: u8) -> Self {
                let bytes = [byte; Self::len()];

                bytes.into()
            }
        }

        impl From<[u8; $nbytes]> for $ty {
            #[inline]
            fn from(value: [u8; $nbytes]) -> Self {
                let slice = Box::leak(Box::new(value));
                let ptr = slice.as_ptr();

                $ty(ptr)
            }
        }

        #[cfg(any(test, feature = "debug"))]
        impl core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                fn fmt_char(byte: u8) -> (char, char) {
                    let msb: u8 = (byte & 0xF0) >> 4;
                    let lsb: u8 = byte & 0x0F;

                    let a = char::from_digit(msb as u32, 16).unwrap();
                    let b = char::from_digit(lsb as u32, 16).unwrap();

                    (a, b)
                }

                let slice = self.as_slice();

                for byte in slice.iter() {
                    let (a, b) = fmt_char(*byte);
                    write!(f, "{}{}", a, b);
                }

                Ok(())
            }
        }

        impl PartialEq for $ty {
            fn eq(&self, other: &$ty) -> bool {
                self.as_slice() == other.as_slice()
            }
        }

        impl Eq for $ty {}
    };
}

impl_blob_type!(Address, 20);
