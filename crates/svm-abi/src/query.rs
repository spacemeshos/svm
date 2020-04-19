use crate::{
    render::VarRenderer,
    schema::{Schema, Var, VarLayout},
};

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
    fn read_var(&mut self, schema: &Schema, req: &StorageReq) -> Option<String> {
        schema.get_var(req.var_id).and_then(|var| {
            self.read_raw_var(&var.layout)
                .and_then(|bytes| VarRenderer::render(&var, &bytes[..]))
        })
    }

    fn read_raw_var(&mut self, layout: &VarLayout) -> Option<Vec<u8>>;
}

impl StorageQuery {
    pub fn new() -> Self {
        Self { reqs: Vec::new() }
    }

    pub fn add_req(&mut self, req: StorageReq) {
        self.reqs.push(req);
    }

    pub fn run<R: StorageReader>(&self, schema: &Schema, storage: &mut R) -> Vec<Option<String>> {
        self.reqs
            .iter()
            .map(|req| storage.read_var(schema, req))
            .collect()
    }
}
