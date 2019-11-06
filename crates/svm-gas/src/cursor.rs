#[derive(Debug, Copy, Clone)]
pub(crate) struct Cursor(usize);

impl Cursor {
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }

    #[inline(always)]
    pub fn forward(&mut self, delta: usize) {
        self.0 += delta;
    }

    #[inline(always)]
    pub fn next(&mut self) {
        self.forward(1);
    }

    #[inline(always)]
    pub fn prev(&mut self) {
        assert!(self.0 > 0);

        self.0 -= 1;
    }

    #[inline(always)]
    pub fn set(&mut self, offset: usize) {
        self.0 = offset;
    }

    #[inline(always)]
    pub fn get(&self) -> usize {
        self.0
    }
}
