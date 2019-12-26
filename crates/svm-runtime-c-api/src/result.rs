#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum svm_result_t {
    SVM_SUCCESS = 0,
    SVM_FAILURE = 1,
}

impl Into<bool> for svm_result_t {
    fn into(self) -> bool {
        match self {
            svm_result_t::SVM_SUCCESS => true,
            svm_result_t::SVM_FAILURE => false,
        }
    }
}

impl svm_result_t {
    #[inline(always)]
    pub fn as_bool(self) -> bool {
        self.into()
    }
}
