use std::iter::Iterator;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct svm_resource_t {
    pub type_id: usize,

    pub count: i32,
}

pub struct Snapshot {
    resources: Vec<svm_resource_t>,
}

impl Snapshot {
    pub fn new(resources: Vec<svm_resource_t>) -> Self {
        Self { resources }
    }

    pub fn into_iter(self) -> impl Iterator<Item = svm_resource_t> {
        self.resources.into_iter()
    }
}
