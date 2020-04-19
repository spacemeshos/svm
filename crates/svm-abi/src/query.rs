use crate::{render::VarRenderer, schema::Var};

pub enum StorageReqKind {
    Get,
}

pub struct StorageReq {
    var_id: usize,

    kind: StorageReqKind,

    params: Vec<String>,
}

pub struct StorageQuery {
    reqs: Vec<StorageReq>,
}

pub trait StorageReader {
    fn read(&mut self, req: &StorageReq) -> Vec<Var>;
}

impl StorageQuery {
    pub fn new() -> Self {
        Self { reqs: Vec::new() }
    }

    pub fn add_req(&mut self, req: StorageReq) {
        self.reqs.push(req);
    }

    pub fn run<R: StorageReader>(&self, storage: &mut R) -> Vec<Vec<Var>> {
        self.reqs.iter().map(|req| storage.read(req)).collect()
    }
}
