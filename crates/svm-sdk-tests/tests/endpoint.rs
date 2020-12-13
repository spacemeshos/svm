#[test]
fn endpoint_tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/endpoint/bool_params.rs");
}
