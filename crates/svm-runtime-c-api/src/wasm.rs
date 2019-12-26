// #[repr(u32)]
// pub enum svm_arg_type {
//     I32 = 0,
//     I64 = 1,
//     Fixed = 2,
//     Slice = 3,
// }
//
// #[repr(u32)]
// pub enum svm_arg_int_type {
//     I32 = 0,
//     I64 = 1,
// }
//
// #[repr(C)]
// pub enum svm_arg_value {
//     I32(i32),
//     I64(i64),
//     Slice,
// }
//
// #[repr(C)]
// pub struct svm_arg_slice {
//     pub bytes: *const u8,
//     pub bytes_len: u32,
// }
