// use std::cell::Cell;
// use std::fmt::{self, Debug, Formatter};
//
// /// `impl_register` macro implements a `SvmRegXXX` register.
// ///
// /// * `bytes_count` - the number of bytes the register holds
// ///
// /// * `reg_ident` - the symbolic name of the register.
// ///   a good practice is to name a register after the pattern `WasmerReg`{bits} register bits.
// ///
// ///  For example:
// /// ```rust
// /// impl_register!(4, SvmReg32);
// /// impl_register!(8, SvmReg64);
// /// ```
// ///
// /// means: `SvmReg64` is a `SVM` register holding 8 bytes (64 bits)
// macro_rules! impl_register {
//     ($bytes_count: expr, $reg_ident: ident) => {
//         /// Implements `SVM` register having `$bytes_count` bytes
//         #[repr(transparent)]
//         #[derive(Clone)]
//         pub struct $reg_ident(pub(crate) [u8; $bytes_count]);
//
//         impl $reg_ident {
//             /// we initialize the register content with zero bytes
//             #[allow(clippy::new_without_default)]
//             #[inline(always)]
//             pub fn new() -> Self {
//                 Self([0; $bytes_count])
//             }
//
//             /// Copies the data given in `cells` into the register content
//             /// Pads the remaining register bytes with zeros (in case `cells.len()` is smaller than the register capacity).
//             #[inline(always)]
//             pub fn copy_from_cells(&mut self, cells: &[Cell<u8>]) {
//                 let padding = $bytes_count as isize - cells.len() as isize;
//
//                 if padding >= 0 {
//                     for (i, cell) in cells.iter().enumerate() {
//                         self.0[i] = cell.get();
//                     }
//
//                     for i in cells.len()..$bytes_count {
//                         self.0[i] = 0;
//                     }
//                 } else {
//                     panic!("`cells` can't fit register");
//                 }
//             }
//
//             /// Copies `count` bytes starting from raw pointer `src`.
//             /// Pads the remaining register bytes with zeros (in case `count` is smaller than the register capacity).
//             pub unsafe fn copy_from(&mut self, src: *const u8, count: u8) {
//                 let count = count as isize;
//                 let padding = $bytes_count as isize - count;
//
//                 if padding >= 0 {
//                     for i in 0..count {
//                         let addr = src.offset(i);
//                         self.0[i as usize] = std::ptr::read(addr);
//                     }
//
//                     for i in count..$bytes_count {
//                         self.0[i as usize] = 0;
//                     }
//                 } else {
//                     panic!("`count` can't fit register");
//                 }
//             }
//
//             /// Copies the data of the register into the input `cells`.
//             /// It works even though we receive `cells` as `&[Cell<u8>]` and not `&mut[Cell<u8>]`
//             /// thanks to the interior mutability of `Cell<T>`
//             #[inline(always)]
//             pub fn copy_to_cells(&self, cells: &[Cell<u8>]) {
//                 for (byte, cell) in self.0.iter().zip(cells) {
//                     cell.set(*byte);
//                 }
//             }
//
//             /// Returns a copy of the register content as a byte array
//             pub fn view(&self) -> Vec<u8> {
//                 self.0.to_vec()
//             }
//
//             /// Returns a pointer to the register underlying content
//             pub unsafe fn as_ptr(&self) -> *const u8 {
//                 self.0.as_ptr()
//             }
//
//             /// Returns a copy of the register first `n` bytes as a byte-array
//             pub fn getn(&self, n: usize) -> Vec<u8> {
//                 assert!(n <= $bytes_count);
//
//                 self.0[0..n].to_vec()
//             }
//
//             /// Overrides the content of the register with the input `bytes` (byte-array).
//             /// Pads remaining bytes with zeros (in case `bytes` is smaller than the register capacity).
//             /// Panics when `bytes` is larger then the register capacity.
//             pub fn set(&mut self, bytes: &[u8]) {
//                 let padding = $bytes_count as isize - bytes.len() as isize;
//
//                 if padding > 0 {
//                     for (i, byte) in bytes.iter().enumerate() {
//                         self.0[i] = *byte;
//                     }
//
//                     // zeroing the remaining bytes
//                     for i in bytes.len()..$bytes_count {
//                         self.0[i] = 0;
//                     }
//                 } else if padding == 0 {
//                     // optimized `set`
//                     self.0.copy_from_slice(&bytes);
//                 } else {
//                     panic!("`bytes` can't fit register");
//                 }
//             }
//
//             /// Replaces a specific register byte with input `byte` at `offset` without affecting the rest.
//             pub fn replace_byte(&mut self, byte: u8, offset: i32) {
//                 assert!(offset >= 0 && offset < $bytes_count);
//
//                 self.0[offset as usize] = byte;
//             }
//         }
//
//         impl Debug for $reg_ident {
//             fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//                 writeln!(f, "{:?}", &self.0[..])
//             }
//         }
//
//         impl PartialEq for $reg_ident {
//             fn eq(&self, other: &Self) -> bool {
//                 &self.0[..] == &other.0[..]
//             }
//         }
//
//         impl std::cmp::PartialOrd for $reg_ident {
//             #[inline(always)]
//             fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//                 self.0.partial_cmp(&other.0)
//             }
//         }
//     };
// }
