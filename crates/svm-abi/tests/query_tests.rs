use serde_json::Value;

use svm_abi::{query::*, render::*, schema::*};

struct StorageMock {
    bytes: Vec<u8>,
}

impl StorageMock {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl StorageReader<Value, JsonVarRenderer> for StorageMock {
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
        let expected = vec![Some($expected)];

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

    test_var!(vec![0], layout, VarType::Bool, Value::Bool(false));
    test_var!(vec![1], layout, VarType::Bool, Value::Bool(true));
}

#[test]
fn query_blob_var() {
    let layout = VarLayout {
        page_idx: 0,
        offset: 0,
        length: 3,
    };

    test_var!(
        vec![10, 20, 30],
        layout,
        VarType::Blob,
        Value::String("0A141E".to_string())
    );
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
        Value::String("0x0A0B0C0D0E".to_string())
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
        Value::String("0x0A0B0C0D0E".to_string())
    );
}
