use std::cell::Cell;
use std::fmt::{self, Debug, Formatter};

macro_rules! impl_register {
    ($bytes_count: expr, $reg_ident: ident) => {
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct $reg_ident(pub(crate) [u8; $bytes_count]);

        impl $reg_ident {
            #[inline(always)]
            pub(crate) fn new() -> Self {
                Self([0; $bytes_count])
            }

            #[inline(always)]
            pub(crate) fn copy_from_wasmer_mem(&mut self, cells: &[Cell<u8>]) {
                for i in 0..$bytes_count {
                    self.0[i] = cells[i].get();
                }
            }

            #[inline(always)]
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
