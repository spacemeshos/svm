use svm_sdk_mock::template;

#[template]
mod Template {
    // The `svm_alloc` is auto-generated.
}

fn main() {
    let size = 10u32;
    let offset: u32 = svm_alloc(size);

    assert!(offset > 0);
}
