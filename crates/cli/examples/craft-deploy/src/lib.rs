use svm_sdk::template;

#[template]
mod Template {
    #[ctor]
    fn and(a: bool, b: bool) -> bool {
        a && b
    }
}
