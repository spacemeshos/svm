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
fn storage_tests() {
    let t = trybuild::TestCases::new();

    compile_fail(&t, "tests/storage/invalid_field.rs");
    compile_fail(&t, "tests/storage/singleton.rs");

    pass(&t, "tests/storage/bool_field.rs");
    pass(&t, "tests/storage/amount_field.rs");
    pass(&t, "tests/storage/address_field.rs");
    pass(&t, "tests/storage/amount_fqn_field.rs");
    pass(&t, "tests/storage/address_fqn_field.rs");
    pass(&t, "tests/storage/u8_field.rs");
    pass(&t, "tests/storage/i8_field.rs");
    pass(&t, "tests/storage/u16_field.rs");
    pass(&t, "tests/storage/i16_field.rs");
    pass(&t, "tests/storage/u32_field.rs");
    pass(&t, "tests/storage/i32_field.rs");
    pass(&t, "tests/storage/u64_field.rs");
    pass(&t, "tests/storage/i64_field.rs");
}
