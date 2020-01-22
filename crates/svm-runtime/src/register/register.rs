use std::ptr;

#[derive(Debug, Clone)]
pub struct Register {
    byte_size: usize,
    current: usize,
    limit: usize,
    data: Vec<u8>,
}

impl Register {
    pub fn new(byte_size: usize, init_cap: usize) -> Self {
        let init_cap = std::cmp::max(1, init_cap);

        Self {
            byte_size,
            current: 0,
            limit: 0,
            data: vec![0; byte_size * init_cap],
        }
    }

    pub fn push(&mut self) {
        let new_current = self.current + self.byte_size;

        if new_current > self.limit {
            let zeros = vec![0; self.byte_size];
            self.data.extend(zeros);

            self.limit = new_current;
        }

        self.current = new_current;
    }

    pub fn pop(&mut self) {
        assert!(self.current >= self.byte_size);

        self.zero();

        self.current -= self.byte_size;
    }

    pub fn view(&self) -> Vec<u8> {
        self.get().to_owned()
    }

    pub fn get(&self) -> &[u8] {
        &self.data[self.current..self.current + self.byte_size]
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        &mut self.data[self.current..self.current + self.byte_size]
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.get().as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.get_mut().as_mut_ptr()
    }

    pub fn set(&mut self, data: &[u8]) {
        let count = data.len();

        self.ensure_fits(count);

        let src = data.as_ptr();
        let dst = self.as_mut_ptr();

        unsafe {
            ptr::copy(src, dst, count);
        }

        // zeroing the remaining register bytes
        let diff = self.byte_size - count;
        if diff > 0 {
            unsafe {
                let dst = dst.offset(count as isize);
                ptr::write_bytes(dst, 0, diff);
            }
        }
    }

    pub fn getn(&self, count: usize) -> &[u8] {
        self.ensure_fits(count);

        let data = self.get();

        &data[0..count]
    }

    pub unsafe fn copy(&mut self, src: *const u8, count: usize) {
        self.ensure_fits(count);

        let dst = self.as_mut_ptr();

        ptr::copy(src, dst, count);

        // zeroing the remaining register data
        let diff = self.byte_size - count;
        if diff > 0 {
            let dst = dst.offset(count as isize);

            ptr::write_bytes(dst, 0, diff);
        }
    }

    #[inline]
    fn ensure_fits(&self, count: usize) {
        assert!(
            self.byte_size >= count,
            format!(
                "`can't fit into register (count=`{}` > register-byte_size=`{}`)",
                count, self.byte_size
            )
        );
    }

    fn zero(&mut self) {
        let dst = self.as_mut_ptr();

        unsafe {
            ptr::write_bytes(dst, 0, self.byte_size);
        }
    }
}

#[allow(unused_comparisons)]
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_zeros {
        ($data:expr) => {{
            assert_zeros!($data, 0, $data.len());
        }};
        ($data:expr, $start:expr, $end:expr) => {{
            assert!($end >= $start);

            let len = $end - $start;

            let zeros = vec![0; len as usize];
            let slice = &$data[$start..$end];

            assert_eq!(zeros.as_slice(), slice);
        }};
    }

    #[test]
    fn register_new_defaults_to_zeros() {
        let reg_size = 8;
        let init_cap = 1;

        let reg = Register::new(reg_size, init_cap);
        assert_zeros!(reg.view());
    }

    #[test]
    fn register_set_exact_register_capcity() {
        let data = vec![10, 20, 30, 40, 50, 60, 70, 80];
        let reg_size = data.len();
        let init_cap = 2;

        let mut reg = Register::new(reg_size, init_cap);
        reg.set(&data[..]);

        assert_eq!(data, reg.view());
        assert_eq!(data, reg.getn(reg_size));
    }

    #[test]
    fn register_set_less_than_register_capacity() {
        let reg_size = 8;
        let init_cap = 2;

        let mut reg = Register::new(reg_size, init_cap);
        reg.set(&vec![0xFF; reg_size]);

        // assert all register bytes are 0xFF
        assert_eq!(vec![0xFF; reg_size], reg.view());

        // now we `set` less than register data on register `0` (which already has data in it)
        let data = vec![20, 30, 40];
        let len = data.len();
        reg.set(&data[..]);

        assert_eq!(data, reg.getn(len));
        assert_zeros!(reg.view(), len, reg_size);
    }

    #[test]
    fn register_set_empty_slice() {
        let reg_size = 8;
        let init_cap = 2;

        let mut reg = Register::new(reg_size, init_cap);
        reg.set(&vec![0xFF; reg_size]);
        assert_eq!(vec![0xFF; reg_size], reg.view());

        reg.set(&vec![]);

        assert_zeros!(reg.getn(0));
        assert_zeros!(reg.view());
    }

    #[test]
    fn register_set_data_larger_than_register_capacity() {
        let reg_size = 8;
        let init_cap = 2;

        let res = std::panic::catch_unwind(|| {
            let mut reg = Register::new(reg_size, init_cap);
            reg.set(&vec![0xFF; reg_size + 1]);
        });

        assert!(res.is_err());
    }

    #[test]
    fn register_as_ptr_defaults_to_zeros() {
        let reg_size = 16;
        let init_cap = 2;

        let reg = Register::new(reg_size, init_cap);
        let ptr = reg.as_ptr();

        for i in 0..reg_size {
            unsafe {
                let addr = ptr.offset(i as isize);
                let byte = std::ptr::read(addr);
                assert_eq!(0, byte);
            }
        }
    }

    #[test]
    fn register_as_ptr() {
        let data = [10, 20, 30, 40, 50, 60, 70, 80];
        let reg_size = data.len();

        let mut reg = Register::new(reg_size, 0);
        reg.set(&data);

        let ptr = reg.as_ptr();

        for i in 0..reg_size {
            let expected = ((i + 1) * 10) as u8;

            unsafe {
                let addr = ptr.offset(i as isize);
                let actual = std::ptr::read(addr);

                assert_eq!(expected, actual);
            }
        }
    }

    #[test]
    fn register_copy_exact_register_capacity() {
        let data = [10, 20, 30, 40, 50, 60, 70, 80];
        let reg_size = data.len();
        let init_cap = 2;

        let mut reg = Register::new(reg_size, init_cap);

        unsafe {
            reg.copy(data.as_ptr(), reg_size);
            assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg.view());
        }
    }

    #[test]
    fn register_copy_from_less_than_register_capacity() {
        let init_cap = 2;
        let reg_size = 8;

        let mut reg = Register::new(reg_size, init_cap);
        reg.set(&vec![0xFF; reg_size]);
        assert_eq!(vec![0xFF; reg_size], reg.view());

        let data = vec![10, 20, 30];
        let len = data.len();

        unsafe { reg.copy(data.as_ptr(), len) };

        assert_eq!(data, reg.getn(len));
        assert_zeros!(reg.view(), len, reg_size);
    }

    #[test]
    fn register_copy_larger_than_register_capacity() {
        let data = [10, 20, 30, 40, 50, 60, 70, 80, 90];
        let reg_size = data.len();
        let init_cap = 2;

        let mut reg = Register::new(reg_size, init_cap);

        let res = std::panic::catch_unwind(move || {
            unsafe { reg.copy(data.as_ptr(), reg_size + 1) };
        });

        assert!(res.is_err());
    }

    #[test]
    fn register_push_within_initial_capacity_limits() {
        let data1 = vec![10, 20, 30, 40, 50, 60, 70, 80];
        let data2 = vec![11, 22, 33, 44, 55, 66, 77, 88];

        let reg_size = data1.len();
        let init_cap = 2;

        let mut reg = Register::new(reg_size, init_cap);
        reg.set(&data1[..]);
        reg.push();

        assert_zeros!(reg.view());

        reg.set(&data2[..]);
        assert_eq!(data2, reg.view());

        reg.pop();
        assert_eq!(data1, reg.view());
    }

    #[test]
    fn register_push_exceeds_initial_capacity_limits() {
        let data1 = vec![10, 20, 30, 40, 50, 60, 70, 80];
        let data2 = vec![11, 22, 33, 44, 55, 66, 77, 88];

        let reg_size = data1.len();
        let init_cap = 1;

        let mut reg = Register::new(reg_size, init_cap);
        reg.set(&data1[..]);
        reg.push();

        assert_zeros!(reg.view());

        reg.set(&data2[..]);
        assert_eq!(data2, reg.view());

        reg.pop();
        assert_eq!(data1, reg.view());
    }

    #[test]
    fn register_push_zeros_register() {
        let data1 = vec![10, 20, 30, 40, 50, 60, 70, 80];
        let data2 = vec![11, 22, 33, 44, 55, 66, 77, 88];

        let reg_size = data1.len();
        let init_cap = 1;

        let mut reg = Register::new(reg_size, init_cap);
        reg.set(&data1[..]);

        reg.push();
        reg.set(&data2[..]);
        assert_eq!(data2, reg.view());

        reg.pop();
        reg.push();
        assert_zeros!(reg.view());
    }

    #[test]
    #[should_panic]
    fn register_pop_more_times_than_push_should_panic() {
        let reg_size = 8;
        let init_cap = 2;

        let mut reg = Register::new(reg_size, init_cap);
        reg.push();
        reg.pop();
        reg.pop();
    }
}
