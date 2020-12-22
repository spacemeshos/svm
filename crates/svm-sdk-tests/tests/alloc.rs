use svm_sdk::host::MockHost;

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
