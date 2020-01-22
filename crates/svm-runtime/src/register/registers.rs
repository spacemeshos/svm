use std::collections::HashMap;

use crate::register::Register;

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

#[derive(Debug)]
pub struct Registers {
    regs: Vec<Register>,
    reg_pos: HashMap<(i32, i32), usize>,
}

impl Default for Registers {
    fn default() -> Self {
        let config = vec![
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

        let regs_capacity = config.iter().map(|(_reg_bits, reg_count)| reg_count).sum();

        let mut regs = Vec::with_capacity(regs_capacity);
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
