#[derive(Debug)]
#[repr(C)]
pub enum svm_result_t {
    SUCCESS = 0,
    FAILURE = 1,
}

impl Into<bool> for svm_result_t {
    fn into(self) -> bool {
        match self {
            svm_result_t::SUCCESS => true,
            svm_result_t::FAILURE => false,
        }
    }
}

impl svm_result_t {
    #[inline(always)]
    pub fn as_bool(self) -> bool {
        self.into()
    }
}
