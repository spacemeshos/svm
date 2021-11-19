extern crate svm_sdk_mock as svm_sdk;

use svm_sdk_mock::host::MockHost;
use svm_sdk_mock::storage::MockStorage;

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
fn ctor_tests() {
    let t = TestCases::new();

    // `#[ctor]` and `#[endpoint]` should behave exactly the same.
    //
    // The difference between the two is that a `ctor` is allowed to execute
    // only when spawning a new `Account`, whilst an `endpoint` is allowed to run only
    // from within an existing `Account`.
    //
    // The job of the `svm-sdk` is only to communicate for which exported function
    // whether it represents an `endpoint` or a `ctor`.
    // This data should be encoded in the `deploy-template` transaction.
    //
    // This restriction should be enforced by the `SVM` Runtime:
    //
    // * When trying to spawn a new `Account` with a public function not defined as `ctor`,
    //   the validation phase should fail the transaction.
    //
    // * When trying to execute a function of an existing `Account` defined as a `ctor`,
    //   the validation phase should fail the transaction.
    pass(&t, "tests/ctor/bool_params.rs");

    compile_fail(&t, "tests/ctor/ctor_and_fundable_attrs_wrong_order.rs");
    compile_fail(&t, "tests/ctor/ctor_used_twice_fails.rs");
}
