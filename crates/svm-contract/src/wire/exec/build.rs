// use crate::wasm::{WasmArgType, WasmArgValue};
// use byteorder::{BigEndian, WriteBytesExt};
// use svm_common::Address;
//
// pub struct WireTxBuilder {
//     version: Option<u32>,
//     contract: Option<Address>,
//     sender: Option<Address>,
//     func_name: Option<String>,
//     func_args: Option<Vec<WasmArgValue>>,
// }
//
// impl WireTxBuilder {
//     pub fn new() -> Self {
//         Self {
//             version: None,
//             contract: None,
//             sender: None,
//             func_name: None,
//             func_args: None,
//         }
//     }
//
//     pub fn with_version(mut self, version: u32) -> Self {
//         self.version = Some(version);
//         self
//     }
//
//     pub fn with_contract(mut self, contract: Address) -> Self {
//         self.contract = Some(contract);
//         self
//     }
//
//     pub fn with_sender(mut self, sender: Address) -> Self {
//         self.sender = Some(sender);
//         self
//     }
//
//     pub fn with_func_name(mut self, func_name: &str) -> Self {
//         self.func_name = Some(func_name.to_string());
//         self
//     }
//
//     pub fn with_func_args(mut self, func_args: &[WasmArgValue]) -> Self {
//         self.func_args = Some(func_args.to_vec());
//         self
//     }
//
//     pub fn build(&mut self) -> Vec<u8> {
//         let mut buf = Vec::new();
//
//         self.write_version(&mut buf);
//         self.write_contract(&mut buf);
//         self.write_sender(&mut buf);
//         self.write_func_name(&mut buf);
//         self.write_func_args(&mut buf);
//
//         buf
//     }
//
//     fn write_version(&self, buf: &mut Vec<u8>) {
//         let version = self.version.unwrap();
//         buf.write_u32::<BigEndian>(version).unwrap();
//     }
//
//     fn write_contract(&self, buf: &mut Vec<u8>) {
//         self.write_address(self.contract.unwrap(), buf);
//     }
//
//     fn write_sender(&self, buf: &mut Vec<u8>) {
//         self.write_address(self.sender.unwrap(), buf);
//     }
//
//     fn write_func_name(&mut self, buf: &mut Vec<u8>) {
//         let name = self.func_name.take().unwrap();
//         let bytes = name.as_bytes();
//
//         assert!(bytes.len() <= 255);
//         buf.write_u8(bytes.len() as u8).unwrap();
//
//         buf.extend_from_slice(bytes);
//     }
//
//     fn write_func_args(&self, buf: &mut Vec<u8>) {
//         let args = self.func_args.as_ref().unwrap();
//
//         buf.write_u8(args.len() as u8).unwrap();
//
//         for arg in args {
//             match arg {
//                 WasmArgValue::I32(v) => {
//                     let arg_type = WasmArgType::I32.into();
//                     buf.write_u8(arg_type).unwrap();
//                     buf.write_u32::<BigEndian>(*v).unwrap();
//                 }
//                 WasmArgValue::I64(v) => {
//                     let arg_type = WasmArgType::I64.into();
//                     buf.write_u8(arg_type).unwrap();
//                     buf.write_u64::<BigEndian>(*v).unwrap();
//                 }
//                 _ => unimplemented!(),
//             }
//         }
//     }
//
//     fn write_address(&self, address: Address, buf: &mut Vec<u8>) {
//         let mut bytes = address.bytes();
//
//         // transforming back into *Big-Endian* order
//         bytes.reverse();
//
//         buf.extend_from_slice(&bytes);
//     }
// }
