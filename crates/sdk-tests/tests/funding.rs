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
fn app_tests() {
    let t = TestCases::new();

    pass(&t, "tests/funding/funding.rs");

    compile_fail(
        &t,
        "tests/funding/fundable_hook_and_fundable_not_allowed.rs",
    );

    compile_fail(&t, "tests/funding/fundable_hook_used_twice_fails.rs");
    compile_fail(&t, "tests/funding/fundable_used_twice_fails.rs");
    compile_fail(&t, "tests/funding/fundable_hook_func_with_args_fails.rs");

    compile_fail(
        &t,
        "tests/funding/fundable_hook_func_with_return_type_fails.rs",
    );
}
