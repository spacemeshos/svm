use svm_sdk::ensure;

/// We stub the `svm_log` external function defined at `log.rs`
#[no_mangle]
fn svm_log(_msg_ptr: u32, _msg_len: u32, _code: u32) {
    //
}

#[ignore]
#[test]
fn ensure_true() {
    ensure!(1 < 2, "one is smaller than two");
}

#[ignore]
#[test]
fn ensure_false() {
    let result = std::panic::catch_unwind(|| {
        ensure!(1 >= 2, "one is smaller than two");
    });

    assert!(result.is_err());
}
