use svm_sdk::host::MockHost;
use trybuild::TestCases;

fn pass(t: &TestCases, test: &'static str) {
    MockHost::reset();

    t.pass(test);
}

fn compile_fail(t: &TestCases, test: &'static str) {
    MockHost::reset();

    t.compile_fail(test);
}

#[test]
fn endpoint_tests() {
    let t = TestCases::new();

    pass(&t, "tests/endpoint/bool_params.rs");
    pass(&t, "tests/endpoint/amount_params.rs");
    pass(&t, "tests/endpoint/address_params.rs");
    pass(&t, "tests/endpoint/integers_params.rs");

    pass(&t, "tests/endpoint/funding.rs");
    compile_fail(&t, "tests/endpoint/endpoint_with_fundable_hook.rs");
}
