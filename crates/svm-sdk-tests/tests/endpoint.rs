use svm_sdk::host::MockHost;
use svm_sdk::storage::MockStorage;

use trybuild::TestCases;

fn pass(t: &TestCases, test: &'static str) {
    MockHost::reset();
    MockStorage::clear();

    t.pass(test);
}

fn compile_fail(t: &TestCases, test: &'static str) {
    MockHost::reset();
    MockStorage::clear();

    t.compile_fail(test);
}

#[test]
fn endpoint_tests() {
    let t = TestCases::new();

    pass(&t, "tests/endpoint/bool_params.rs");
    pass(&t, "tests/endpoint/amount_params.rs");
    pass(&t, "tests/endpoint/address_params.rs");
    pass(&t, "tests/endpoint/integers_params.rs");

    compile_fail(&t, "tests/endpoint/endpoint_used_twice_fails.rs");
    compile_fail(&t, "tests/endpoint/endpoint_and_ctor_fails.rs");
    compile_fail(&t, "tests/endpoint/endpoint_with_fundable_hook.rs");

    compile_fail(
        &t,
        "tests/endpoint/endpoint_and_fundable_attrs_wrong_order.rs",
    );
}
