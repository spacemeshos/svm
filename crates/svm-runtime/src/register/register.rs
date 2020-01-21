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
        } else {
            // no need to allocate more data for the register.
            // zero-ing the register data.
            let dst = self.as_mut_ptr();

            unsafe {
                ptr::write_bytes(dst, 0, self.byte_size);
            }
        }

        self.current = new_current;
    }

    pub fn pop(&mut self) {
        assert!(self.current >= self.byte_size);

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

    pub fn as_ptr(&mut self) -> *const u8 {
        self.get().as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.get_mut().as_mut_ptr()
    }

    pub fn set(&mut self, data: &[u8]) {
        let src = data.as_ptr();
        let dst = self.as_mut_ptr();

        unsafe {
            ptr::copy(src, dst, self.byte_size);
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

        // zeroing the remaining register bytes
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
}
