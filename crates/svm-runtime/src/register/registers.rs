use std::collections::HashMap;

use crate::register::Register;

const REGS_128_COUNT: usize = 8;
const REGS_160_COUNT: usize = 8;
const REGS_256_COUNT: usize = 4;
const REGS_512_COUNT: usize = 4;

use lazy_static::lazy_static;

lazy_static! {
    static ref REGS_DEFAULT_CONFIG: Vec<(usize, usize)> = vec![
        (128, REGS_128_COUNT),
        (160, REGS_160_COUNT),
        (256, REGS_256_COUNT),
        (512, REGS_512_COUNT),
    ];
}

#[derive(Debug)]
pub struct Registers {
    regs: Vec<Register>,
    reg_pos: HashMap<(i32, i32), usize>,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new(&REGS_DEFAULT_CONFIG[..])
    }
}

impl Registers {
    pub fn new(config: &[(usize, usize)]) -> Self {
        let mut index = 0;

        let regs_cap = config.iter().map(|(_reg_bits, reg_count)| reg_count).sum();

        let mut regs = Vec::with_capacity(regs_cap);
        let mut reg_pos = HashMap::new();

        for &(reg_bits, reg_count) in config.iter() {
            assert!(reg_bits % 8 == 0);

            let reg_size = reg_bits / 8;
            let inital_cap = 4;

            for reg_idx in 0..reg_count {
                reg_pos.insert((reg_bits as i32, reg_idx as i32), index);

                let reg = Register::new(reg_size, inital_cap);
                regs.push(reg);

                index += 1;
            }
        }

        Registers { regs, reg_pos }
    }

    #[inline]
    pub fn get_reg(&self, reg_bits: i32, reg_idx: i32) -> &Register {
        let pos = self.reg_pos(reg_bits, reg_idx);

        &self.regs[pos]
    }

    #[inline]
    pub fn get_reg_mut(&mut self, reg_bits: i32, reg_idx: i32) -> &mut Register {
        let pos = self.reg_pos(reg_bits, reg_idx);

        &mut self.regs[pos]
    }

    #[inline]
    fn reg_pos(&self, reg_bits: i32, reg_idx: i32) -> usize {
        *self.reg_pos.get(&(reg_bits, reg_idx)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_get_valid_reg() {
        let config = [(128, 2), (256, 3)];
        let mut regs = Registers::new(&config);

        let data1 = vec![0xAA; 16];
        let data2 = vec![0xBB; 16];
        let data3 = vec![0xCC; 32];

        let reg128_0 = regs.get_reg_mut(128, 0);
        reg128_0.set(&data1[..]);

        let reg128_1 = regs.get_reg_mut(128, 1);
        reg128_1.set(&data2[..]);

        let reg256_0 = regs.get_reg_mut(256, 0);
        reg256_0.set(&data3[..]);

        let reg128_0 = regs.get_reg(128, 0);
        let reg128_1 = regs.get_reg(128, 1);
        let reg256_0 = regs.get_reg(256, 0);

        assert_eq!(data1, reg128_0.view());
        assert_eq!(data2, reg128_1.view());
        assert_eq!(data3, reg256_0.view());
    }

    fn registers_get_out_of_bounds_reg_panics() {
        let reg_bits = 128;
        let reg_count = 10;

        let config = [(reg_bits, reg_count)];
        let mut regs = Registers::new(&config);

        for reg_idx in 0..reg_count {
            let _reg = regs.get_reg(reg_bits as i32, reg_idx as i32);
        }

        let res = std::panic::catch_unwind(move || {
            let _reg = regs.get_reg(reg_bits as i32, reg_count as i32);
        });

        assert!(res.is_err());
    }
}
