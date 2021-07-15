use svm_sdk::template;

#[link(wasm_import_module = "host")]
extern "C" {
    fn host_fail();
}

#[template]
mod Template {
    #[ctor]
    fn initialize() {}

    #[endpoint]
    fn fail() {
        unsafe { host_fail() }
    }
}
