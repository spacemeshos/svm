use std::cell::Cell;
use std::fmt::{self, Debug, Formatter};

/// `impl_register` macro implements a `wasmer register`.
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
            /// we initialize the register content with zero bytes
            #[inline(always)]
            pub(crate) fn new() -> Self {
                Self([0; $bytes_count])
            }

            /// Copies the data given in `cells` into the register content
            #[inline(always)]
            pub(crate) fn copy_from_wasmer_mem(&mut self, cells: &[Cell<u8>]) {
                let padding = $bytes_count as isize - cells.len() as isize;

                if padding >= 0 {
                    for (i, cell) in cells.iter().enumerate() {
                        self.0[i] = cell.get();
                    }

                    for i in cells.len()..$bytes_count {
                        self.0[i] = 0;
                    }
                } else {
                    panic!("`cells` can't fit register");
                }
            }

            /// Copies the data of the register into the input `cells`.
            /// It works even though we receive `cells` as `&[Cell<u8>]` and not `&mut[Cell<u8>]`
            /// thanks to the interior mutability of `Cell<T>`
            #[inline(always)]
            pub(crate) fn copy_to_wasmer_mem(&self, cells: &[Cell<u8>]) {
                for (byte, cell) in self.0.iter().zip(cells) {
                    cell.set(*byte);
                }
            }

            /// Returns a copy of the register content as a byte array
            pub(crate) fn get(&self) -> [u8; $bytes_count] {
                self.0
            }

            /// Returns a copy of the register first `n` bytes as a byte-array
            pub(crate) fn getn(&self, n: usize) -> Vec<u8> {
                assert!(n <= $bytes_count);

                self.0[0..n].to_vec()
            }

            /// Overrides the content of the register with the input `bytes` (byte-array).
            /// Pads remaining bytes with `0` in case `bytes` is small than the register capacity.
            /// Panics when `bytes` is larger then the register capacity.
            pub(crate) fn set(&mut self, bytes: &[u8]) {
                let padding = $bytes_count as isize - bytes.len() as isize;

                if padding > 0 {
                    for (i, byte) in bytes.iter().enumerate() {
                        self.0[i] = *byte;
                    }

                    // zeroing the remaining bytes
                    for i in bytes.len()..$bytes_count {
                        self.0[i] = 0;
                    }
                } else if padding == 0 {
                    // optimized `set`
                    self.0.copy_from_slice(&bytes);
                } else {
                    panic!("`bytes` can't fit register");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_defaults_to_zeros() {
        let reg = WasmerReg64::new();

        assert_eq!([0; 8], reg.get());
    }

    #[test]
    fn copy_from_wasmer_mem_exact_register_capacity() {
        let cells = [
            Cell::new(10),
            Cell::new(20),
            Cell::new(30),
            Cell::new(40),
            Cell::new(50),
            Cell::new(60),
            Cell::new(70),
            Cell::new(80),
        ];

        let mut reg = WasmerReg64::new();
        assert_eq!([0; 8], reg.get());

        reg.copy_from_wasmer_mem(&cells);

        assert_eq!([10, 20, 30, 40, 50, 60, 70, 80], reg.get());
    }

    #[test]
    fn copy_from_wasmer_mem_less_than_register_capacity() {
        let mut reg = WasmerReg64::new();
        reg.set(&vec![10; 8]);
        assert_eq!([10; 8], reg.get());

        let cells = [Cell::new(10), Cell::new(20), Cell::new(30)];

        reg.copy_from_wasmer_mem(&cells);

        assert_eq!(vec![10, 20, 30], reg.getn(3));
        assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg.get());
    }

    // #[test]
    // #[should_panic]
    // fn copy_from_wasmer_mem_bigger_than_register_capacity() {
    //     let mut reg = WasmerReg64::new();
    //     let cells = [
    //         Cell::new(10),
    //         Cell::new(20),
    //         Cell::new(30),
    //         Cell::new(40),
    //         Cell::new(50),
    //         Cell::new(60),
    //         Cell::new(70),
    //         Cell::new(80),
    //         Cell::new(90),
    //     ];
    //
    //     reg.copy_from_wasmer_mem(&cells);
    // }

    #[test]
    fn set_exact_register_capcity() {
        let mut reg = WasmerReg64::new();
        assert_eq!([0; 8], reg.get());

        reg.set(&vec![10, 20, 30, 40, 50, 60, 70, 80]);

        assert_eq!([10, 20, 30, 40, 50, 60, 70, 80], reg.get());
        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.getn(8));
    }

    #[test]
    fn set_less_than_register_capacity() {
        let mut reg = WasmerReg64::new();
        reg.set(&vec![10; 8]);
        assert_eq!([10; 8], reg.get());

        // now we `set` less than register bytes on register `0` (which already has data in it)
        reg.set(&vec![20, 30, 40]);

        assert_eq!(vec![20, 30, 40], reg.getn(3));
        assert_eq!([20, 30, 40, 0, 0, 0, 0, 0], reg.get());
    }

    #[test]
    fn set_empty_slice() {
        let mut reg = WasmerReg64::new();
        reg.set(&vec![10; 8]);
        assert_eq!([10; 8], reg.get());

        // now we `set` [] on register `0` (which already has data in it)
        reg.set(&vec![]);

        assert_eq!(Vec::<u8>::new(), reg.getn(0));
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0], reg.get());
    }

    // #[test]
    // #[should_panic(expected = "`bytes` can't fit register")]
    // fn setting_data_larger_than_register_capacity_raises() {
    //     let mut reg = WasmerReg64::new();
    //     reg.set(&vec![10; 9]);
    // }
}
