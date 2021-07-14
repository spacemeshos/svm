/// `impl_bytes_primitive` macro implements a struct consisting of one array of bytes.
#[macro_export]
macro_rules! impl_bytes_primitive {
    ($primitive: ident, $byte_count: expr) => {
        /// `$primitive` consists of `$byte_count` bytes.
        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $primitive(pub(self) [u8; $byte_count]);

        impl From<[u8; $byte_count]> for $primitive {
            fn from(data: [u8; $byte_count]) -> $primitive {
                $primitive(data)
            }
        }

        impl From<&[u8]> for $primitive {
            fn from(slice: &[u8]) -> $primitive {
                assert_eq!($byte_count, slice.len());

                let mut buf: [u8; $byte_count] = [0; $byte_count];
                buf.copy_from_slice(slice);

                $primitive(buf)
            }
        }

        impl From<*const u8> for $primitive {
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            fn from(ptr: *const u8) -> $primitive {
                let slice: &[u8] = unsafe { std::slice::from_raw_parts(ptr, $byte_count) };

                $primitive::from(slice)
            }
        }

        impl From<*mut u8> for $primitive {
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            #[inline]
            fn from(ptr: *mut u8) -> $primitive {
                $primitive::from(ptr as *const u8)
            }
        }

        impl From<*const std::ffi::c_void> for $primitive {
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            #[inline]
            fn from(ptr: *const std::ffi::c_void) -> $primitive {
                $primitive::from(ptr as *const u8)
            }
        }

        impl $primitive {
            /// Returns a raw pointer into the internal byte-array
            pub fn as_ptr(&self) -> *const u8 {
                self.0.as_ptr()
            }

            /// Returns a slice into the internal byte-array
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }

            /// Returns a clone of the internal internal array
            pub fn bytes(&self) -> [u8; $byte_count] {
                self.0.clone()
            }

            /// Returns a String representation
            pub fn as_str(&self) -> String {
                hex::encode_upper(self.0)
            }

            /// Generates a new instance with all-zeros data.
            pub fn zeros() -> Self {
                Self::repeat(0)
            }

            /// Returns whether the underlying data is all-zeros
            pub fn is_zeros(&self) -> bool {
                self.0 == [0; $byte_count]
            }

            /// Generates an instance where all the bytes equal `byte`
            /// This method is very useful for generating data for tests.
            pub fn repeat(byte: u8) -> Self {
                let bytes = [byte; $byte_count];

                Self(bytes)
            }

            /// # Safety
            ///
            /// Decomposes into its raw components.
            pub unsafe fn into_raw_parts(self) -> (*mut u8, usize, usize) {
                let vec = self.0.to_vec();

                vec.into_raw_parts()
            }

            /// Returns an `iter` over the underlying bytes
            pub fn iter(&self) -> std::slice::Iter<u8> {
                self.0.iter()
            }

            /// Returns the first `n` number of bytes
            pub fn first_n(&self, n: usize) -> Vec<u8> {
                assert!(n <= $byte_count);

                self.as_slice()[0..n].to_vec()
            }

            /// Returns the last `n` number of bytes
            pub fn last_n(&self, n: usize) -> Vec<u8> {
                assert!(n <= $byte_count);

                self.iter().skip($byte_count - n).cloned().collect()
            }

            /// Returns the number of bytes
            #[inline]
            pub const fn len() -> usize {
                $byte_count
            }

            /// formats the primitive as a concatenation of:
            /// * first `first` bytes in hex
            /// * ...
            /// * last `last` bytes in hex
            pub fn fmt(&self, first: usize, last: usize) -> String {
                let first = self.first_n(first);
                let last = self.last_n(last);

                format!("{}...{}", hex::encode_upper(first), hex::encode_upper(last),)
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
    };
}
