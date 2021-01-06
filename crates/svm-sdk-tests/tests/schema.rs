use svm_sdk::host::MockHost;
use svm_sdk::storage::MockStorage;

use trybuild::TestCases;

fn pass(t: &TestCases, test: &'static str) {
    MockHost::reset();
    MockStorage::clear();

    t.pass(test);
}

#[test]
fn schema_tests() {
    let t = TestCases::new();

    pass(&t, "tests/schema/storage_schema.rs");

    pass(&t, "tests/schema/ctor_schema.rs");
    pass(&t, "tests/schema/ctor_fundable_schema.rs");

    pass(&t, "tests/schema/endpoint_fundable_schema.rs");
    pass(&t, "tests/schema/endpoint_with_params_schema.rs");
    pass(&t, "tests/schema/endpoint_with_params_array_schema.rs");

    pass(&t, "tests/schema/endpoint_with_returns_tuple_schema.rs");
    pass(&t, "tests/schema/endpoint_with_returns_path_schema.rs");
}
