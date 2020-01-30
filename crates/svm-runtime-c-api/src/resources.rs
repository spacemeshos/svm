use std::collections::HashMap;

use crate::svm_byte_array;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ResourceKind {
    Address = 0,
    State = 1,
    Receipt = 2,
}

pub type ResourceId = u8;

pub struct Resource {
    used: bool,
    id: ResourceId,
    kind: ResourceKind,
    data: svm_byte_array,
}

pub struct Resources {
    resources: HashMap<ResourceKind, HashMap<ResourceId, Resource>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn checkout(&mut self, kind: ResourceKind) -> Resource {
        todo!()
    }

    pub fn checkin(&mut self, resource: Resource) {
        todo!()
    }

    fn resources(&self, kind: ResourceKind) -> &Vec<(bool, svm_byte_array)> {
        todo!()
    }
}
