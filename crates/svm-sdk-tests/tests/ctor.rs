use svm_sdk::host::MockHost;
use svm_sdk::storage::MockStorage;

use trybuild::TestCases;

fn pass(t: &TestCases, test: &'static str) {
    MockHost::reset();
    MockStorage::clear();

    t.pass(test);
}

#[test]
fn ctor_tests() {
    let t = TestCases::new();

    // `#[ctor]` and `#[endpoint]` should behave exactly the same.
    //
    // The difference between the two is that a `ctor` is allowed to execute
    // only when spawning a new app, whilst an `endpoint` is allowed to run only
    // from within an existing `app`.
    //
    // The job of the `svm-sdk` is only to communicate for which exported function
    // whether it represents an `endpoint` or a `ctor`.
    // This data should be encoded in the `deploy-template` transaction.
    //
    // This restriction should be enforced by the `SVM` Runtime:
    //
    // * When trying to spawn a new `app` with a public function not defined as `ctor`
    // in the `deploy-template` - the validation phase should fail the transaction.
    //
    // * When trying to execute a function of an existing `app` defined as a `ctor`
    // in the `deploy-template` - the validation phase should fail the transaction.
    pass(&t, "tests/ctor/bool_params.rs");
}
