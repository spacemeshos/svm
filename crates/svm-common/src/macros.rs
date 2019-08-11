/// `impl_bytes_primitive` macro implements a struct consisting of one
#[macro_export]
macro_rules! impl_bytes_primitive {
    ($primitive: ident, $bytes_count: expr) => {
        /// Spacemesh `$primitive` primitive consists of $bytes_count bytes.
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $primitive(pub [u8; $bytes_count]);

        impl From<*const u8> for $primitive {
            fn from(ptr: *const u8) -> $primitive {
                let slice: &[u8] = unsafe { std::slice::from_raw_parts(ptr, $bytes_count) };

                let mut buf: [u8; $bytes_count] = [0; $bytes_count];
                buf.copy_from_slice(slice);

                $primitive(buf)
            }
        }

        impl $primitive {
            /// Returns a raw pointer into the `$primitive` internal array
            pub fn as_ptr(&self) -> *const u8 {
                self.0.as_ptr()
            }
        }
    };
}
