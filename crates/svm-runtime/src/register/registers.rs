use std::collections::HashMap;

use crate::register::Register;

static REGS_32_COUNT: usize = 16;
static REGS_64_COUNT: usize = 16;
static REGS_128_COUNT: usize = 8;
static REGS_160_COUNT: usize = 8;
static REGS_256_COUNT: usize = 4;
static REGS_512_COUNT: usize = 4;

// static REGS_DEFAULT_CONFIG: Vec<(usize, usize)> = vec![
//     (32, REGS_32_COUNT),
//     (64, REGS_64_COUNT),
//     (128, REGS_128_COUNT),
//     (160, REGS_160_COUNT),
//     (256, REGS_256_COUNT),
//     (512, REGS_512_COUNT),
// ];

pub struct Registers {
    regs: Vec<Register>,
    reg_index: HashMap<(i32, i32), usize>,
}

impl Default for Registers {
    fn default() -> Self {
        let config = vec![
            (32, REGS_32_COUNT),
            (64, REGS_64_COUNT),
            (128, REGS_128_COUNT),
            (160, REGS_160_COUNT),
            (256, REGS_256_COUNT),
            (512, REGS_512_COUNT),
        ];

        Self::new(&config[..])
    }
}

impl Registers {
    pub fn new(config: &[(usize, usize)]) -> Self {
        let mut index = 0;
        let mut cap = 0;

        let regs_count = config.iter().map(|(_, reg_count)| reg_count).sum();

        let mut regs = Registers {
            regs: Vec::with_capacity(regs_count),
            reg_index: HashMap::new(),
        };

        for &(reg_bits, reg_count) in config.iter() {
            assert!(reg_bits % 8 == 0);
            let reg_size = reg_bits / 8;
            let inital_cap = 2;

            for reg_idx in 0..reg_count {
                regs.reg_index
                    .insert((reg_bits as i32, reg_idx as i32), index);

                let reg = Register::new(reg_size, inital_cap);
                regs.regs.push(reg);

                index += 1;
            }
        }

        regs
    }

    #[inline(always)]
    pub fn get_reg(&self, reg_bits: i32, reg_idx: i32) -> &Register {
        todo!()
    }

    #[inline(always)]
    pub fn get_reg_mut(&self, reg_bits: i32, reg_idx: i32) -> &mut Register {
        todo!()
    }

    #[inline(always)]
    pub fn set_reg(&self, reg_bits: i32, reg_idx: i32, data: &[u8]) {
        todo!()
    }

    #[inline(always)]
    fn reg_index(&self, reg_bits: i32, reg_idx: i32) -> usize {
        *self.reg_index.get(&(reg_bits, reg_idx)).unwrap()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn reg_defaults_to_zeros() {
//         let reg = Registers::new();
//
//         assert_eq!(vec![0; 8], reg.view());
//     }
//
//     #[test]
//     fn as_ptr_defaults_to_zeros() {
//         let reg = SvmReg64::new();
//         let ptr = unsafe { reg.as_ptr() };
//
//         for i in 0..8 {
//             let addr = unsafe { ptr.offset(i) };
//             let byte = unsafe { std::ptr::read(addr) };
//             assert_eq!(0, byte);
//         }
//     }
//
//     #[test]
//     fn as_ptr() {
//         let cells = [
//             Cell::new(10),
//             Cell::new(20),
//             Cell::new(30),
//             Cell::new(40),
//             Cell::new(50),
//             Cell::new(60),
//             Cell::new(70),
//             Cell::new(80),
//         ];
//
//         let mut reg = SvmReg64::new();
//         let ptr = unsafe { reg.as_ptr() };
//
//         assert_eq!(vec![0; 8], reg.view());
//
//         reg.copy_from_cells(&cells);
//
//         for i in 0..8 {
//             let expected = (i + 1) * 10 as u8;
//
//             let addr = unsafe { ptr.offset(i as isize) };
//             let actual = unsafe { std::ptr::read(addr) };
//
//             assert_eq!(expected, actual);
//         }
//     }
//
//     #[test]
//     fn copy_from_exact_register_capacity() {
//         let data = [10, 20, 30, 40, 50, 60, 70, 80];
//
//         let mut reg = SvmReg64::new();
//         assert_eq!(vec![0; 8], reg.view());
//
//         unsafe { reg.copy_from(data.as_ptr(), 8) };
//         assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.view());
//     }
//
//     #[test]
//     fn copy_from_less_than_register_capacity() {
//         let mut reg = SvmReg64::new();
//         reg.set(&vec![10; 8]);
//         assert_eq!(vec![10; 8], reg.view());
//
//         let data: Vec<u8> = vec![10, 20, 30];
//
//         unsafe { reg.copy_from(data.as_ptr(), 3) };
//
//         assert_eq!(vec![10, 20, 30], reg.getn(3));
//         assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
//     }
//
//     #[test]
//     #[ignore]
//     fn copy_from_cells_slice_larger_than_register_capacity() {
//         let mut reg = SvmReg64::new();
//
//         let data = [10, 20, 30, 40, 50, 60, 70, 80, 90];
//
//         let res = std::panic::catch_unwind(move || {
//             unsafe { reg.copy_from(data.as_ptr(), 9) };
//         });
//
//         assert!(res.is_err());
//     }
//
//     #[test]
//     fn copy_from_cells_slice_of_exact_same_lengthh_as_register_capacity() {
//         let cells = [
//             Cell::new(10),
//             Cell::new(20),
//             Cell::new(30),
//             Cell::new(40),
//             Cell::new(50),
//             Cell::new(60),
//             Cell::new(70),
//             Cell::new(80),
//         ];
//
//         let mut reg = SvmReg64::new();
//         assert_eq!(vec![0; 8], reg.view());
//
//         reg.copy_from_cells(&cells);
//         assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.view());
//     }
//
//     #[test]
//     fn copy_from_cells_slice_shorter_than_register_capacity() {
//         let mut reg = SvmReg64::new();
//         reg.set(&vec![10; 8]);
//         assert_eq!(vec![10; 8], reg.view());
//
//         let cells = [Cell::new(10), Cell::new(20), Cell::new(30)];
//
//         reg.copy_from_cells(&cells);
//
//         assert_eq!(vec![10, 20, 30], reg.getn(3));
//         assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
//     }
//
//     #[test]
//     fn copy_from_cells_slice_longer_than_register_capacity() {
//         let mut reg = SvmReg64::new();
//
//         let cells = [
//             Cell::new(10),
//             Cell::new(20),
//             Cell::new(30),
//             Cell::new(40),
//             Cell::new(50),
//             Cell::new(60),
//             Cell::new(70),
//             Cell::new(80),
//             Cell::new(90),
//         ];
//
//         let res = std::panic::catch_unwind(move || {
//             reg.copy_from_cells(&cells);
//         });
//
//         assert!(res.is_err());
//     }
//
//     #[test]
//     fn set_exact_register_capcity() {
//         let mut reg = SvmReg64::new();
//         assert_eq!(vec![0; 8], reg.view());
//
//         reg.set(&vec![10, 20, 30, 40, 50, 60, 70, 80]);
//
//         assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.view());
//         assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.getn(8));
//     }
//
//     #[test]
//     fn set_less_than_register_capacity() {
//         let mut reg = SvmReg64::new();
//         reg.set(&vec![10; 8]);
//         assert_eq!(vec![10; 8], reg.view());
//
//         // now we `set` less than register bytes on register `0` (which already has data in it)
//         reg.set(&vec![20, 30, 40]);
//
//         assert_eq!(vec![20, 30, 40], reg.getn(3));
//         assert_eq!(vec![20, 30, 40, 0, 0, 0, 0, 0], reg.view());
//     }
//
//     #[test]
//     fn set_empty_slice() {
//         let mut reg = SvmReg64::new();
//         reg.set(&vec![10; 8]);
//         assert_eq!(vec![10; 8], reg.view());
//
//         // now we `set` [] on register `0` (which already has data in it)
//         reg.set(&vec![]);
//
//         assert_eq!(Vec::<u8>::new(), reg.getn(0));
//         assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0], reg.view());
//     }
//
//     #[test]
//     fn set_data_larger_than_register_capacity_raises() {
//         let res = std::panic::catch_unwind(|| {
//             let mut reg = SvmReg64::new();
//             reg.set(&vec![10; 9]);
//         });
//
//         assert!(res.is_err());
//     }
// }
