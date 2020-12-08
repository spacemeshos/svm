use svm_sdk::app;

#[test]
fn storage_tests() {
    let t = trybuild::TestCases::new();
    t.pass("storage-tests/bool_field.rs");
}
