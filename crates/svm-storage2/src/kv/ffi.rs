// use svm_kv::traits::KVStore;

// type GetFn = unsafe extern "C" fn(*const u8, u32, *mut u32) -> *const u8;
// type SetFn = unsafe extern "C" fn(*const u8, u32);
// type CommitFn = unsafe extern "C" fn();

// pub struct ExternKV {
//     get_fn: GetFn,
//     set_fn: SetFn,
//     commit_fn: CommitFn,
// }

// impl ExternKV {
//     pub fn new(get_fn: GetFn, set_fn: SetFn, commit_fn: CommitFn) -> Self {
//         Self {
//             get_fn,
//             set_fn,
//             commit_fn,
//         }
//     }
// }

// impl KVStore for ExternKV {
//     fn get(&self, ns: &[u8], key: &[u8]) -> Option<Vec<u8>> {
//         let key_ptr = key.as_ptr();
//         let key_len = key.len() as u32;

//         let mut value_len = 0;

//         let value_ptr = unsafe { (self.get_fn)(key_ptr, key_len, &mut value_len) };

//         if value_len > 0 {
//             let value = unsafe { std::slice::from_raw_parts(value_ptr, value_len as usize) };

//             Some(value.to_vec())
//         } else {
//             None
//         }
//     }

//     fn set(&mut self, changes: &[(Vec<u8>, Vec<u8>)]) {
//         for (k, v) in changes.iter() {
//             let key_ptr = k.as_ptr();
//             let key_len = k.len() as u32;

//             unsafe {
//                 (self.set_fn)(key_ptr, key_len);
//             }
//         }

//         unsafe {
//             (self.commit_fn)();
//         }
//     }
// }
