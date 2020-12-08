#[test]
fn storage_tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/storage-tests/bool_field.rs");
}
