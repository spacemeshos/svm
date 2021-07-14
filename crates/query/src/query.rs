//! Contains Query-related stuff.

use crate::{
    render::VarRenderer,
    schema::{Schema, Var},
};

/// Request kind represents the fetching method used
/// for executing the request.
///
/// Note: currently only the `Get` kind is supported.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StorageReqKind {
    /// Simple `Get` request.
    Get,
}

/// A request for retrieving a variable.
///
#[derive(Debug, Clone, PartialEq)]
pub struct StorageReq {
    /// The variable id
    pub var_id: usize,

    /// The kind of request
    pub kind: StorageReqKind,

    /// Parameters for the request
    pub params: Vec<String>,
}

/// A `StorageQuery` contains a batch of requests.
pub struct StorageQuery {
    reqs: Vec<StorageReq>,
}

/// This trait should be implemented by App's storage interfaces.
///
/// See `AppStorage` under the `svm-storage` crate.
pub trait StorageReader<V, VR: VarRenderer<V>> {
    /// Executes a read request.
    ///
    /// First, reads its raw data by calling `read_raw`.
    /// Then, renders the raw data into a `String`.
    fn read_var(&mut self, schema: &Schema, req: &StorageReq) -> Option<V> {
        let var = schema.var(req.var_id);

        var.and_then(|v| {
            let bytes = self.read_var_raw(req, &v);

            bytes.and_then(|b| VR::render(&v, &b[..]))
        })
    }

    /// Retrieves the raw data of a request.
    fn read_var_raw(&mut self, req: &StorageReq, var: &Var) -> Option<Vec<u8>>;
}

impl StorageQuery {
    /// New query.
    pub fn new() -> Self {
        Self { reqs: Vec::new() }
    }

    /// Adds a request to the query
    pub fn add_req(&mut self, req: StorageReq) {
        self.reqs.push(req);
    }

    /// Executes batch of query's requests.
    ///
    /// Returns a Vector such that the `i-th` item is the result of executing the `i-th` request.
    ///
    /// Note:
    /// Each request is returned as `Option<String>`.
    /// If a request returned `None` is means that something is was wrong with the request.
    /// It may be due to not in-sync Storage ABI.
    pub fn run<V, VR, R: StorageReader<V, VR>>(
        &self,
        schema: &Schema,
        storage: &mut R,
    ) -> Vec<Option<V>>
    where
        VR: VarRenderer<V>,
    {
        self.reqs
            .iter()
            .map(|req| storage.read_var(schema, req))
            .collect()
    }
}
