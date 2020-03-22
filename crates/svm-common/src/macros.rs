/// `impl_bytes_primitive` macro implements a struct consisting of one array of bytes.
#[macro_export]
macro_rules! impl_bytes_primitive {
    ($primitive: ident, $byte_count: expr) => {
        /// Spacemesh `$primitive` consists of `$byte_count` bytes.
        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $primitive(pub(self) [u8; $byte_count]);

        impl From<&[u8]> for $primitive {
            fn from(slice: &[u8]) -> $primitive {
                assert_eq!($byte_count, slice.len());

                let mut buf: [u8; $byte_count] = [0; $byte_count];
                buf.copy_from_slice(slice);

                $primitive(buf)
            }
        }

        impl From<*const u8> for $primitive {
            #[warn(clippy::not_unsafe_ptr_arg_deref)]
            fn from(ptr: *const u8) -> $primitive {
                let slice: &[u8] = unsafe { std::slice::from_raw_parts(ptr, $byte_count) };

                $primitive::from(slice)
            }
        }

        impl From<*mut u8> for $primitive {
            #[warn(clippy::not_unsafe_ptr_arg_deref)]
            #[inline]
            fn from(ptr: *mut u8) -> $primitive {
                $primitive::from(ptr as *const u8)
            }
        }

        impl From<*const std::ffi::c_void> for $primitive {
            #[warn(clippy::not_unsafe_ptr_arg_deref)]
            #[inline]
            fn from(ptr: *const std::ffi::c_void) -> $primitive {
                $primitive::from(ptr as *const u8)
            }
        }

        impl $primitive {
            /// Returns a raw pointer into the `$primitive` internal array
            pub fn as_ptr(&self) -> *const u8 {
                self.0.as_ptr()
            }

            /// Returns a slice into the `$primitive` internal array
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }

            /// Returns a clone of the `$primitive` internal array
            pub fn bytes(&self) -> [u8; $byte_count] {
                self.0.clone()
            }

            /// Decomposes a `$primitive` into its raw components
            pub unsafe fn into_raw_parts(self) -> (*mut u8, usize, usize) {
                let mut vec = self.0.to_vec();
                vec.truncate(Self::len());

                vec.into_raw_parts()
            }

            /// Returns an `iter` over the underlying bytes
            pub fn iter(&self) -> std::slice::Iter<u8> {
                self.0.iter()
            }

            /// Returns the first `n` number of bytes of `$primitive`
            pub fn first_n(&self, n: usize) -> Vec<u8> {
                assert!(n <= $byte_count);

                self.as_slice()[0..n].to_vec()
            }

            /// Returns the last `n` number of bytes of `$primitive`
            pub fn last_n(&self, n: usize) -> Vec<u8> {
                assert!(n <= $byte_count);

                self.iter().skip($byte_count - n).cloned().collect()
            }

            /// Returns the number of bytes of `$primitive`
            #[inline]
            pub fn len() -> usize {
                $byte_count
            }

            /// formats the primitive as a concatenation of:
            /// * first `first` bytes in hex
            /// * ...
            /// * last `last` bytes in hex
            pub fn fmt(&self, first: usize, last: usize, separator: &str) -> String {
                let first = self.first_n(first);
                let last = self.last_n(last);

                format!(
                    "{} ... {}",
                    crate::fmt::fmt_hex(first.as_slice(), separator),
                    crate::fmt::fmt_hex(last.as_slice(), separator)
                )
            }

            /// Should be used **only** for tests
            pub fn of(s: &str) -> $primitive {
                let mut buf = [0; $byte_count];

                let bytes = s.as_bytes();

                assert!(bytes.len() <= $byte_count);

                unsafe {
                    std::ptr::copy(bytes.as_ptr(), buf.as_mut_ptr(), bytes.len());
                }

                $primitive(buf)
            }
        }

        /// Should be used **only** for tests
        #[doc(hidden)]
        impl From<u32> for $primitive {
            fn from(n: u32) -> $primitive {
                let mut buf = [0; $byte_count];

                let [n3, n2, n1, n0] = $crate::helpers::u32_to_be_array(n);

                buf[$byte_count - 4] = n3;
                buf[$byte_count - 3] = n2;
                buf[$byte_count - 2] = n1;
                buf[$byte_count - 1] = n0;

                $primitive(buf)
            }
        }

        /// Should be used **only** for tests
        #[doc(hidden)]
        impl From<i32> for $primitive {
            #[inline]
            fn from(n: i32) -> $primitive {
                $primitive::from(n as u32)
            }
        }

        /// Should be used **only** for tests
        #[doc(hidden)]
        impl From<u64> for $primitive {
            fn from(n: u64) -> $primitive {
                let mut buf = [0; $byte_count];

                let [n7, n6, n5, n4, n3, n2, n1, n0] = $crate::helpers::u64_to_be_array(n);

                buf[$byte_count - 8] = n7;
                buf[$byte_count - 7] = n6;
                buf[$byte_count - 6] = n5;
                buf[$byte_count - 5] = n4;
                buf[$byte_count - 4] = n3;
                buf[$byte_count - 3] = n2;
                buf[$byte_count - 2] = n1;
                buf[$byte_count - 1] = n0;

                $primitive(buf)
            }
        }
    };
}
