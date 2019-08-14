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
/// impl_register!(8, SvmReg64);
/// ```
///
/// means: `SvmReg64` is a `wasmer` register holding 8 bytes (64 bits)
macro_rules! impl_register {
    ($bytes_count: expr, $reg_ident: ident) => {
        /// Implements a `wasmer svm` register of $bytes_count bytes
        #[repr(transparent)]
        #[derive(Clone)]
        pub struct $reg_ident(pub(crate) [u8; $bytes_count]);

        impl $reg_ident {
            /// we initialize the register content with zero bytes
            #[inline(always)]
            pub fn new() -> Self {
                Self([0; $bytes_count])
            }

            /// Copies the data given in `cells` into the register content
            /// Pads the remaining register bytes with zeros (in case `cells.len()` is smaller than the register capacity).
            #[inline(always)]
            pub fn copy_from_wasmer_mem(&mut self, cells: &[Cell<u8>]) {
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

            /// Copies `count` bytes starting from raw pointer `src`.
            /// Pads the remaining register bytes with zeros (in case `count` is smaller than the register capacity).
            pub unsafe fn copy_from(&mut self, src: *const u8, count: u8) {
                let count = count as isize;
                let padding = $bytes_count as isize - count;

                if padding >= 0 {
                    for i in 0..count {
                        let addr = src.offset(i);
                        self.0[i as usize] = std::ptr::read(addr);
                    }

                    for i in count..$bytes_count {
                        self.0[i as usize] = 0;
                    }
                } else {
                    panic!("`count` can't fit register");
                }
            }

            /// Copies the data of the register into the input `cells`.
            /// It works even though we receive `cells` as `&[Cell<u8>]` and not `&mut[Cell<u8>]`
            /// thanks to the interior mutability of `Cell<T>`
            #[inline(always)]
            pub fn copy_to_wasmer_mem(&self, cells: &[Cell<u8>]) {
                for (byte, cell) in self.0.iter().zip(cells) {
                    cell.set(*byte);
                }
            }

            /// Returns a copy of the register content as a byte array
            pub fn view(&self) -> Vec<u8> {
                self.0.to_vec()
            }

            /// Returns a pointer to the register underlying content
            pub unsafe fn as_ptr(&self) -> *const u8 {
                self.0.as_ptr()
            }

            /// Returns a copy of the register first `n` bytes as a byte-array
            pub fn getn(&self, n: usize) -> Vec<u8> {
                assert!(n <= $bytes_count);

                self.0[0..n].to_vec()
            }

            /// Overrides the content of the register with the input `bytes` (byte-array).
            /// Pads remaining bytes with zeros (in case `bytes` is smaller than the register capacity).
            /// Panics when `bytes` is larger then the register capacity.
            pub fn set(&mut self, bytes: &[u8]) {
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

        impl std::cmp::PartialOrd for $reg_ident {
            #[inline(always)]
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
    };
}

impl_register!(8, SvmReg64);
impl_register!(20, SvmReg160);
impl_register!(32, SvmReg256);
impl_register!(64, SvmReg512);

/// A `SvmRegXXX` wrapper. Used in order to avoid `mismatched types` under macros `match` arms.
/// Another alternative to address that problem, might have been to add a trait for the `SvmRegXXX` methods and then use a trait object.
pub enum SvmReg {
    /// Wrapper for a `SvmReg64` register
    Reg64(SvmReg64),

    /// Wrapper for a `SvmReg160` register
    Reg160(SvmReg160),

    /// Wrapper for a `SvmReg256` register
    Reg256(SvmReg256),

    /// Wrapper for a `SvmReg512` register
    Reg512(SvmReg512),
}

impl SvmReg {
    /// Delegates `copy_from_wasmer_mem` to the inner wrapped `SvmRegXXX`
    #[inline(always)]
    pub fn copy_from_wasmer_mem(&mut self, cells: &[Cell<u8>]) {
        match self {
            SvmReg::Reg64(reg) => reg.copy_from_wasmer_mem(cells),
            SvmReg::Reg160(reg) => reg.copy_from_wasmer_mem(cells),
            SvmReg::Reg256(reg) => reg.copy_from_wasmer_mem(cells),
            SvmReg::Reg512(reg) => reg.copy_from_wasmer_mem(cells),
        }
    }

    /// Delegates `copy_from` to the inner wrapped `SvmRegXXX`
    #[inline(always)]
    pub unsafe fn copy_from(&mut self, src: *const u8, count: u8) {
        match self {
            SvmReg::Reg64(reg) => reg.copy_from(src, count),
            SvmReg::Reg160(reg) => reg.copy_from(src, count),
            SvmReg::Reg256(reg) => reg.copy_from(src, count),
            SvmReg::Reg512(reg) => reg.copy_from(src, count),
        }
    }

    /// Delegates `as_ptr` to the inner wrapped `SvmRegXXX`
    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const u8 {
        match self {
            SvmReg::Reg64(reg) => reg.as_ptr(),
            SvmReg::Reg160(reg) => reg.as_ptr(),
            SvmReg::Reg256(reg) => reg.as_ptr(),
            SvmReg::Reg512(reg) => reg.as_ptr(),
        }
    }

    /// Delegates `set` to the inner wrapped `SvmRegXXX`
    #[inline(always)]
    pub fn set(&mut self, bytes: &[u8]) {
        match self {
            SvmReg::Reg64(reg) => reg.set(bytes),
            SvmReg::Reg160(reg) => reg.set(bytes),
            SvmReg::Reg256(reg) => reg.set(bytes),
            SvmReg::Reg512(reg) => reg.set(bytes),
        }
    }

    /// Delegates `copy_to_wasmer_mem` to the inner wrapped `SvmRegXXX`
    #[inline(always)]
    pub fn copy_to_wasmer_mem(&self, cells: &[Cell<u8>]) {
        match self {
            SvmReg::Reg64(reg) => reg.copy_to_wasmer_mem(cells),
            SvmReg::Reg160(reg) => reg.copy_to_wasmer_mem(cells),
            SvmReg::Reg256(reg) => reg.copy_to_wasmer_mem(cells),
            SvmReg::Reg512(reg) => reg.copy_to_wasmer_mem(cells),
        }
    }

    /// Delegates `getn` to the inner wrapped `SvmRegXXX`
    #[inline(always)]
    pub fn getn(&self, n: usize) -> Vec<u8> {
        match self {
            SvmReg::Reg64(reg) => reg.getn(n),
            SvmReg::Reg160(reg) => reg.getn(n),
            SvmReg::Reg256(reg) => reg.getn(n),
            SvmReg::Reg512(reg) => reg.getn(n),
        }
    }

    /// Delegates `view` to the inner wrapped `SvmRegXXX`
    #[inline(always)]
    pub fn view(&self) -> Vec<u8> {
        match self {
            SvmReg::Reg64(reg) => reg.view(),
            SvmReg::Reg160(reg) => reg.view(),
            SvmReg::Reg256(reg) => reg.view(),
            SvmReg::Reg512(reg) => reg.view(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_defaults_to_zeros() {
        let reg = SvmReg64::new();

        assert_eq!(vec![0; 8], reg.view());
    }

    #[test]
    fn as_ptr_defaults_to_zeros() {
        let reg = SvmReg64::new();
        let ptr = unsafe { reg.as_ptr() };

        for i in 0..8 {
            let addr = unsafe { ptr.offset(i) };
            let byte = unsafe { std::ptr::read(addr) };
            assert_eq!(0, byte);
        }
    }

    #[test]
    fn as_ptr() {
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

        let mut reg = SvmReg64::new();
        let ptr = unsafe { reg.as_ptr() };

        assert_eq!(vec![0; 8], reg.view());

        reg.copy_from_wasmer_mem(&cells);

        for i in 0..8 {
            let expected = (i + 1) * 10 as u8;

            let addr = unsafe { ptr.offset(i as isize) };
            let actual = unsafe { std::ptr::read(addr) };

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn copy_from_exact_register_capacity() {
        let data = [10, 20, 30, 40, 50, 60, 70, 80];

        let mut reg = SvmReg64::new();
        assert_eq!(vec![0; 8], reg.view());

        unsafe { reg.copy_from(data.as_ptr(), 8) };
        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.view());
    }

    #[test]
    fn copy_from_less_than_register_capacity() {
        let mut reg = SvmReg64::new();
        reg.set(&vec![10; 8]);
        assert_eq!(vec![10; 8], reg.view());

        let data: Vec<u8> = vec![10, 20, 30];

        unsafe { reg.copy_from(data.as_ptr(), 3) };

        assert_eq!(vec![10, 20, 30], reg.getn(3));
        assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
    }

    #[test]
    #[ignore]
    fn copy_from_bigger_than_register_capacity() {
        let mut reg = SvmReg64::new();

        let data = [10, 20, 30, 40, 50, 60, 70, 80, 90];

        let res = std::panic::catch_unwind(move || {
            unsafe { reg.copy_from(data.as_ptr(), 9) };
        });

        assert!(res.is_err());
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

        let mut reg = SvmReg64::new();
        assert_eq!(vec![0; 8], reg.view());

        reg.copy_from_wasmer_mem(&cells);
        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.view());
    }

    #[test]
    fn copy_from_wasmer_mem_less_than_register_capacity() {
        let mut reg = SvmReg64::new();
        reg.set(&vec![10; 8]);
        assert_eq!(vec![10; 8], reg.view());

        let cells = [Cell::new(10), Cell::new(20), Cell::new(30)];

        reg.copy_from_wasmer_mem(&cells);

        assert_eq!(vec![10, 20, 30], reg.getn(3));
        assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
    }

    #[test]
    fn copy_from_wasmer_mem_bigger_than_register_capacity() {
        let mut reg = SvmReg64::new();

        let cells = [
            Cell::new(10),
            Cell::new(20),
            Cell::new(30),
            Cell::new(40),
            Cell::new(50),
            Cell::new(60),
            Cell::new(70),
            Cell::new(80),
            Cell::new(90),
        ];

        let res = std::panic::catch_unwind(move || {
            reg.copy_from_wasmer_mem(&cells);
        });

        assert!(res.is_err());
    }

    #[test]
    fn set_exact_register_capcity() {
        let mut reg = SvmReg64::new();
        assert_eq!(vec![0; 8], reg.view());

        reg.set(&vec![10, 20, 30, 40, 50, 60, 70, 80]);

        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.view());
        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.getn(8));
    }

    #[test]
    fn set_less_than_register_capacity() {
        let mut reg = SvmReg64::new();
        reg.set(&vec![10; 8]);
        assert_eq!(vec![10; 8], reg.view());

        // now we `set` less than register bytes on register `0` (which already has data in it)
        reg.set(&vec![20, 30, 40]);

        assert_eq!(vec![20, 30, 40], reg.getn(3));
        assert_eq!(vec![20, 30, 40, 0, 0, 0, 0, 0], reg.view());
    }

    #[test]
    fn set_empty_slice() {
        let mut reg = SvmReg64::new();
        reg.set(&vec![10; 8]);
        assert_eq!(vec![10; 8], reg.view());

        // now we `set` [] on register `0` (which already has data in it)
        reg.set(&vec![]);

        assert_eq!(Vec::<u8>::new(), reg.getn(0));
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0], reg.view());
    }

    #[test]
    fn set_data_larger_than_register_capacity_raises() {
        let res = std::panic::catch_unwind(|| {
            let mut reg = SvmReg64::new();
            reg.set(&vec![10; 9]);
        });

        assert!(res.is_err());
    }
}
