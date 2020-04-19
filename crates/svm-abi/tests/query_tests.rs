use svm_abi;

use svm_abi::{query::*, render::*, schema::*};

struct StorageMock {
    bytes: Vec<u8>,
}

impl StorageMock {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl StorageReader for StorageMock {
    fn read_raw_var(&mut self, layout: &VarLayout) -> Option<Vec<u8>> {
        let bool_byte = self.bytes[0];

        Some(vec![bool_byte])
    }
}

#[test]
fn query_bool_var() {
    let var = Var {
        id: 0,
        layout: VarLayout {
            page_idx: 0,
            offset: 0,
            length: 1,
        },
        ty: VarType::Bool,
        name: "var_bool".to_string(),
        desc: "...".to_string(),
    };

    let req = StorageReq {
        var_id: var.id,
        kind: StorageReqKind::Get,
        params: Vec::new(),
    };

    let mut schema = Schema::new();
    schema.add_var(var);

    let mut query = StorageQuery::new();
    query.add_req(req);

    // `False`
    let bytes = vec![0];
    let mut storage = StorageMock::new(bytes);
    let out = query.run(&schema, &mut storage);
    assert_eq!(out, [Some("False".to_string())]);

    // `True`
    let bytes = vec![1];
    let mut storage = StorageMock::new(bytes);
    let out = query.run(&schema, &mut storage);
    assert_eq!(out, [Some("True".to_string())]);
}
