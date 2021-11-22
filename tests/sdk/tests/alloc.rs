extern crate svm_sdk_mock as svm_sdk;

use svm_sdk_mock::host::MockHost;

use trybuild::TestCases;

fn pass(t: &TestCases, test: &'static str) {
    MockHost::reset();

    t.pass(test);
}

#[test]
fn alloc_tests() {
    let t = TestCases::new();

    pass(&t, "tests/alloc/alloc.rs");
}
