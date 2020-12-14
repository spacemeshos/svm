#[test]
fn endpoint_tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/endpoint/bool_params.rs");
    t.pass("tests/endpoint/amount_params.rs");
    t.pass("tests/endpoint/address_params.rs");
    t.pass("tests/endpoint/integers_params.rs");
    t.pass("tests/endpoint/funding.rs");
}
