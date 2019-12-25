/// `impl_bytes_primitive` macro implements a struct consisting of one array of bytes.
#[macro_export]
macro_rules! impl_bytes_primitive {
    ($primitive: ident, $bytes_count: expr) => {
        /// Spacemesh `$primitive` consists of `$bytes_count` bytes.
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $primitive(pub(self) [u8; $bytes_count]);

        impl From<&[u8]> for $primitive {
            fn from(slice: &[u8]) -> $primitive {
                assert_eq!($bytes_count, slice.len());

                let mut buf: [u8; $bytes_count] = [0; $bytes_count];
                buf.copy_from_slice(slice);

                $primitive(buf)
            }
        }

        impl From<*const u8> for $primitive {
            #[warn(clippy::not_unsafe_ptr_arg_deref)]
            fn from(ptr: *const u8) -> $primitive {
                let slice: &[u8] = unsafe { std::slice::from_raw_parts(ptr, $bytes_count) };

                $primitive::from(slice)
            }
        }

        impl From<*const std::ffi::c_void> for $primitive {
            #[warn(clippy::not_unsafe_ptr_arg_deref)]
            #[inline(always)]
            fn from(ptr: *const std::ffi::c_void) -> $primitive {
                $primitive::from(ptr as *const u8)
            }
        }

        impl $primitive {
            /// Returns a raw pointer into the `$primitive` internal array
            pub fn as_ptr(&self) -> *const u8 {
                self.0.as_ptr()
            }

            /// Consumes the `$primitive` object and transfers ownership to a C caller
            pub fn into_raw(self) -> *mut $primitive {
                let boxed = Box::new(self);
                Box::into_raw(boxed)
            }

            /// Retakes ownership of a `$primitive` that was transferred to C via `into_raw`
            pub unsafe fn from_raw(ptr: *mut $primitive) -> $primitive {
                let boxed: Box<$primitive> = Box::from_raw(ptr);
                *boxed
            }

            /// Returns a slice into the `$primitive` internal array
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }

            /// Returns a clone of the `$primitive` internal array
            pub fn bytes(&self) -> [u8; $bytes_count] {
                self.0
            }

            /// Returns the number of bytes of `$primitive`
            #[inline(always)]
            pub fn len() -> usize {
                $bytes_count
            }
        }

        /// Should be used **only** for tests
        #[doc(hidden)]
        impl From<u32> for $primitive {
            fn from(n: u32) -> $primitive {
                let mut buf = [0; $bytes_count];

                let [n3, n2, n1, n0] = $crate::helpers::u32_to_be_array(n);

                buf[$bytes_count - 4] = n3;
                buf[$bytes_count - 3] = n2;
                buf[$bytes_count - 2] = n1;
                buf[$bytes_count - 1] = n0;

                $primitive(buf)
            }
        }

        /// Should be used **only** for tests
        #[doc(hidden)]
        impl From<i32> for $primitive {
            #[inline(always)]
            fn from(n: i32) -> $primitive {
                $primitive::from(n as u32)
            }
        }

        /// Should be used **only** for tests
        #[doc(hidden)]
        impl From<u64> for $primitive {
            fn from(n: u64) -> $primitive {
                let mut buf = [0; $bytes_count];

                let [n7, n6, n5, n4, n3, n2, n1, n0] = $crate::helpers::u64_to_be_array(n);

                buf[$bytes_count - 8] = n7;
                buf[$bytes_count - 7] = n6;
                buf[$bytes_count - 6] = n5;
                buf[$bytes_count - 5] = n4;
                buf[$bytes_count - 4] = n3;
                buf[$bytes_count - 3] = n2;
                buf[$bytes_count - 2] = n1;
                buf[$bytes_count - 1] = n0;

                $primitive(buf)
            }
        }
    };
}
