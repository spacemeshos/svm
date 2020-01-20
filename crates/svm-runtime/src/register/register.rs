use std::ptr;

#[derive(Debug, Clone)]
pub struct Register {
    size: usize,
    current: usize,
    limit: usize,
    data: Vec<u8>,
}

impl Register {
    pub fn new(size: usize, init_cap: usize) -> Self {
        Self {
            size,
            current: 0,
            limit: 0,
            data: vec![0; size * init_cap],
        }
    }

    pub fn push(&mut self) {
        let new_current = self.current + self.size;

        if new_current > self.limit {
            let zeros = vec![0; self.size];
            self.data.extend(zeros);

            self.limit = new_current;
        } else {
            // no need to allocate more data for the register.
            // zero-ing the register data.
            let dst = self.get_mut();

            unsafe {
                ptr::write_bytes::<u8>(dst.as_mut_ptr(), 0, self.size);
            }
        }

        self.current = new_current;
    }

    pub fn pop(&mut self) {
        assert!(self.current >= self.size);

        self.current -= self.size;
    }

    pub fn get(&self) -> &[u8] {
        &self.data[self.current..self.current + self.size]
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        &mut self.data[self.current..self.current + self.size]
    }

    pub fn set(&mut self, src: &[u8]) {
        let dst = self.get_mut();

        unsafe {
            ptr::copy(src.as_ptr(), dst.as_mut_ptr(), self.size);
        }
    }

    pub fn getn(&self, count: usize) -> &[u8] {
        self.ensure_fits(count);

        let data = self.get();

        &data[0..count]
    }

    pub unsafe fn copy(&mut self, src: *const u8, count: usize) {
        self.ensure_fits(count);

        let dst = self.get_mut().as_mut_ptr();

        ptr::copy(src, dst, count);

        // zeroing the remaining register bytes
        let diff = self.size - count;
        if diff > 0 {
            let dst = dst.offset(count as isize);

            ptr::write_bytes::<u8>(dst, 0, diff);
        }
    }

    #[inline]
    fn ensure_fits(&self, count: usize) {
        assert!(
            self.size >= count,
            format!(
                "`can't fit into register (count=`{}` > register-size=`{}`)",
                count, self.size
            )
        );
    }
}
