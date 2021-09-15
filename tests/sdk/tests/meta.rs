use svm_sdk::host::MockHost;
use svm_sdk::storage::MockStorage;

use trybuild::TestCases;

fn pass(t: &TestCases, test: &'static str) {
    MockHost::reset();
    MockStorage::clear();

    t.pass(test);
}

#[test]
fn meta_tests() {
    let t = TestCases::new();

    pass(&t, "tests/meta/storage_meta.rs");

    pass(&t, "tests/meta/ctor_meta.rs");
    pass(&t, "tests/meta/ctor_fundable_meta.rs");

    pass(&t, "tests/meta/ctor_with_doc.rs");
    pass(&t, "tests/meta/endpoint_with_doc.rs");

    pass(&t, "tests/meta/endpoint_fundable_meta.rs");
    pass(&t, "tests/meta/endpoint_with_params_meta.rs");
    pass(&t, "tests/meta/endpoint_with_params_array_meta.rs");

    pass(&t, "tests/meta/endpoint_with_returns_tuple_meta.rs");
    pass(&t, "tests/meta/endpoint_with_returns_path_meta.rs");
}
