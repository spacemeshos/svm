#[test]
fn storage_tests() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/storage/singleton.rs");

    t.pass("tests/storage/bool_field.rs");
    t.pass("tests/storage/amount_field.rs");
    t.pass("tests/storage/address_field.rs");
    t.pass("tests/storage/amount_fqn_field.rs");
    t.pass("tests/storage/address_fqn_field.rs");
    t.pass("tests/storage/u8_field.rs");
    t.pass("tests/storage/i8_field.rs");
    t.pass("tests/storage/u16_field.rs");
    t.pass("tests/storage/i16_field.rs");
    t.pass("tests/storage/u32_field.rs");
    t.pass("tests/storage/i32_field.rs");
    t.pass("tests/storage/u64_field.rs");
    t.pass("tests/storage/i64_field.rs");
}
