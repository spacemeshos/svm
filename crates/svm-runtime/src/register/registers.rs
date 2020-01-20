use std::collections::HashMap;

struct Register {}

pub struct Registers {
    data: Vec<u8>,

    offsets: HashMap<(i32, i32), usize>,
}

const REGS_32_COUNT: usize = 16;
const REGS_64_COUNT: usize = 16;
const REGS_128_COUNT: usize = 8;
const REGS_160_COUNT: usize = 8;
const REGS_256_COUNT: usize = 4;
const REGS_512_COUNT: usize = 4;

const REGS_DEFAULT_CONFIG: Vec<(i32, usize)> = vec![
    (32, REGS_32_COUNT),
    (64, REG_64_COUNT),
    (128, REGS_128_COUNT),
    (160, REG_160_COUNT),
    (256, REG_256_COUNT),
    (512, REG_512_COUNT),
];

impl Default for Registers {
    fn default() -> Self {
        Self::new(&REGS_DEFAULT_CONFIG)
    }
}

impl Registers {
    pub fn new(config: &[(i32, usize)]) -> Self {
        let mut offset = 0;
        let mut cap = 0;
        let mut offsets = HashMap::new();

        for (reg_bits, reg_count) in config.iter() {
            // TODO:
            // assert!(reg_count <= 32);

            for reg_idx in 0..reg_count {
                offsets.insert((reg_bits, reg_idx), offset);
                offset += reg_bits;
            }

            cap += (reg_bits * reg_count);
        }

        Self {
            data: vec![0; cap],
            offsets,
        }
    }

    #[inline(always)]
    pub fn get_reg(&self, reg_bits: i32, reg_idx: i32) -> &[u8] {
        let off = self.reg_offset(reg_bits, reg_ids);
        &[data..data + reg_bits]
    }

    #[inline(always)]
    pub fn get_reg_mut(&self, reg_bits: i32, reg_idx: i32) -> &mut [u8] {
        let off = self.reg_offset(reg_bits, reg_ids);
        &mut [data..data + reg_bits]
    }

    #[inline(always)]
    pub fn set_reg(&self, reg_bits: i32, reg_idx: i32, data: &[u8]) {
        assert!(data.len() <= reg_len(reg_bits));

        let reg = self.get_reg_mut(reg_bits, reg_idx);

        std::ptr::copy(data.as_ptr(), reg.as_mut_ptr(), self.reg_len(reg_bits));
    }

    #[inline(always)]
    pub fn reg_offset(&self, reg_bits: i32, reg_idx: i32) -> usize {
        self.offsets.get(&(reg_bits, reg_idx)).unwrap()
    }

    #[inline(always)]
    fn reg_len(&self, reg_bits: i32) -> usize {
        assert!(reg_bits % 8 == 0);
        reg_bits / 8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reg_defaults_to_zeros() {
        let reg = Registers::new();

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

        reg.copy_from_cells(&cells);

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
    fn copy_from_cells_slice_larger_than_register_capacity() {
        let mut reg = SvmReg64::new();

        let data = [10, 20, 30, 40, 50, 60, 70, 80, 90];

        let res = std::panic::catch_unwind(move || {
            unsafe { reg.copy_from(data.as_ptr(), 9) };
        });

        assert!(res.is_err());
    }

    #[test]
    fn copy_from_cells_slice_of_exact_same_lengthh_as_register_capacity() {
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

        reg.copy_from_cells(&cells);
        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.view());
    }

    #[test]
    fn copy_from_cells_slice_shorter_than_register_capacity() {
        let mut reg = SvmReg64::new();
        reg.set(&vec![10; 8]);
        assert_eq!(vec![10; 8], reg.view());

        let cells = [Cell::new(10), Cell::new(20), Cell::new(30)];

        reg.copy_from_cells(&cells);

        assert_eq!(vec![10, 20, 30], reg.getn(3));
        assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
    }

    #[test]
    fn copy_from_cells_slice_longer_than_register_capacity() {
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
            reg.copy_from_cells(&cells);
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
