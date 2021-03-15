use svm_sdk::app;

#[link(wasm_import_module = "host")]
extern "C" {
    fn host_fail();
}

#[app]
mod App {
    #[ctor]
    fn initialize() {}

    #[endpoint]
    fn fail() {
        unsafe { host_fail() }
    }
}
