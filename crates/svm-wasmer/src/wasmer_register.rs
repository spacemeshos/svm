use std::cell::Cell;
use std::fmt::{self, Debug, Formatter};

/// `impl_register` macro implements a `wasmer register`.
///
/// input params:
///
/// * `bytes_count` - the number of bytes the register holds
///
/// * `reg_ident` - the symbolic name of the register.
///   a good practice is to name a register after the pattern `WasmerReg`{bits} register bits.
///
///  For example:
/// ```rust
/// impl_register!(8, WasmerReg64);
/// ```
///
/// means: `WasmerReg64` is a `wasmer` register holding 8 bytes (64 bits)
macro_rules! impl_register {
    ($bytes_count: expr, $reg_ident: ident) => {
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct $reg_ident(pub(crate) [u8; $bytes_count]);

        impl $reg_ident {
            #[inline(always)]
            /// we initialize the register content with zero bytes
            pub(crate) fn new() -> Self {
                Self([0; $bytes_count])
            }

            #[inline(always)]
            /// Copies the data given in `cells` into the register content
            pub(crate) fn copy_from_wasmer_mem(&mut self, cells: &[Cell<u8>]) {
                for i in 0..$bytes_count {
                    self.0[i] = cells[i].get();
                }
            }

            #[inline(always)]
            /// Copies the data of the register into the input `cells`.
            /// It works even though we receive `cells` as `&[Cell<u8>]` and not `&mut[Cell<u8>]`
            /// thanks to the interior mutability of `Cell<T>`
            pub(crate) fn copy_to_wasmer_mem(&self, cells: &[Cell<u8>]) {
                for (byte, cell) in self.0.iter().zip(cells) {
                    cell.set(*byte);
                }
            }
        }

        impl Debug for $reg_ident {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                writeln!(f, "{:?}", &self.0[..])
            }
        }

        impl PartialEq for $reg_ident {
            fn eq(&self, other: &Self) -> bool {
                &self.0[..] == &other.0[..]
            }
        }
    };
}

impl_register!(8, WasmerReg64);
