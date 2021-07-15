use svm_sdk::template;

#[template]
mod Template {
    #[fundable_hook(default)]
    fn allow() {}

    #[fundable_hook(default)]
    fn deny() {
        panic!()
    }
}

fn main() {}
