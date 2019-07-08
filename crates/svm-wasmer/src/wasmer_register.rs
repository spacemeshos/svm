use std::cell::Cell;
use wasmer_runtime_core::memory::{Memory, MemoryView};

macro_rules! impl_register {
    ($bytes_count: expr, $reg_ident: ident) => {
        #[repr(transparent)]
        pub struct $reg_ident([u8; $bytes_count]);

        impl $reg_ident {
            #[inline(always)]
            fn copy_from_wasmer_mem(&mut self, cells: &[Cell<u8>]) {
                for i in 0..$bytes_count {
                    self.0[i] = cells[i].get();;
                }
            }

            #[inline(always)]
            fn copy_to_wasmer_mem(&self, mem: &mut Memory) {
                // for byte in self.0.iter() {}
            }
        }
    };
}

impl_register!(8, WasmerReg64);
impl_register!(128, WasmerReg128);
