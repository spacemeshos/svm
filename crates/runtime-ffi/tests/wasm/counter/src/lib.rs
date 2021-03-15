use svm_sdk::app;

#[link(wasm_import_module = "host")]
extern "C" {
    fn counter_mul(var_id: u32, mul_by: u32) -> u32;
}

#[app]
mod App {
    #[storage]
    struct Storage {
        counter: u32, // var_id = 0
    }

    #[ctor]
    fn initialize(init: u32) {
        Storage::set_counter(init);
    }

    #[endpoint]
    fn add_and_mul(add: u32, mul: u32) -> [u32; 3] {
        const VAR_ID: u32 = 0;

        let a = Storage::get_counter();
        let b = a + add;

        Storage::set_counter(b);

        let c = unsafe { counter_mul(VAR_ID, mul) };

        Storage::set_counter(c);

        [a, b, c]
    }
}
