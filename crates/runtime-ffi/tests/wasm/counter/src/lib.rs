use svm_sdk::template;

#[template]
mod Template {
    #[storage]
    struct Storage {
        counter: u32,
    }

    #[ctor]
    fn initialize(init: u32) {
        Storage::set_counter(init);
    }

    #[endpoint]
    fn add(n: u32) -> [u32; 2] {
        let a = Storage::get_counter();
        let b = a + n;

        Storage::set_counter(b);

        [a, b]
    }

    #[endpoint]
    fn mul(n: u32) -> [u32; 2] {
        let a = Storage::get_counter();
        let b = a * n;

        Storage::set_counter(b);

        [a, b]
    }
}
