use svm_abi;

use svm_abi::{query::*, schema::*};

struct StorageMock {
    bytes: Vec<u8>,
}

impl StorageMock {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl StorageReader for StorageMock {
    fn read_var_raw(&mut self, _req: &StorageReq, var: &Var) -> Option<Vec<u8>> {
        let layout = &var.layout;

        let start = layout.offset;
        let end = start + layout.length;

        Some(self.bytes[start..end].to_vec())
    }
}

macro_rules! test_var {
    ($bytes:expr, $layout:expr, $ty:path, $expected:expr) => {{
        let var = Var {
            id: 0,
            layout: $layout.clone(),
            ty: $ty,
            name: "...".to_string(),
            desc: "...".to_string(),
        };

        let req = StorageReq {
            var_id: var.id,
            params: Vec::new(),
            kind: StorageReqKind::Get,
        };

        let mut schema = Schema::new();
        schema.add_var(var);

        let mut query = StorageQuery::new();
        query.add_req(req);

        let mut storage = StorageMock::new($bytes);

        let actual = query.run(&schema, &mut storage);
        let expected = vec![Some($expected.to_string())];

        assert_eq!(expected, actual);
    }};
}

#[test]
fn query_bool_var() {
    let layout = VarLayout {
        page_idx: 0,
        offset: 0,
        length: 1,
    };

    test_var!(vec![0], layout, VarType::Bool, "False");
    test_var!(vec![1], layout, VarType::Bool, "True");
}

#[test]
fn query_blob_var() {
    let layout = VarLayout {
        page_idx: 0,
        offset: 0,
        length: 3,
    };

    test_var!(vec![10, 20, 30], layout, VarType::Blob, "0A141E");
}

#[test]
fn query_pubkey_var() {
    let layout = VarLayout {
        page_idx: 0,
        offset: 2,
        length: 5,
    };

    test_var!(
        vec![08, 09, 10, 11, 12, 13, 14],
        layout,
        VarType::PubKey,
        "0x0A0B0C0D0E"
    );
}

#[test]
fn query_addr_var() {
    let layout = VarLayout {
        page_idx: 0,
        offset: 2,
        length: 5,
    };

    test_var!(
        vec![08, 09, 10, 11, 12, 13, 14],
        layout,
        VarType::Address,
        "0x0A0B0C0D0E"
    );
}
