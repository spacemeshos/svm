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
fn template_tests() {
    let t = TestCases::new();

    pass(&t, "tests/template/empty.rs");

    compile_fail(&t, "tests/template/declaring_const_not_allowed.rs");
    compile_fail(&t, "tests/template/declaring_static_not_allowed.rs");
    compile_fail(&t, "tests/template/declaring_enum_not_allowed.rs");
    compile_fail(&t, "tests/template/declaring_union_not_allowed.rs");
    compile_fail(&t, "tests/template/declaring_traits_not_allowed.rs");

    compile_fail(
        &t,
        "tests/template/template_with_two_default_fundable_hook_not_allowed.rs",
    );

    compile_fail(&t, "tests/template/using_extern_crate_not_allowed.rs");
    compile_fail(&t, "tests/template/using_ffi_not_allowed.rs");
    compile_fail(&t, "tests/template/using_impl_not_allowed.rs");
}
