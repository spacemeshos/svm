use svm_sdk::app;

#[app]
mod App {
    #[endpoint]
    fn and(a: bool, b: bool) -> bool {
        a && b
    }
}

fn call<T>(args: Vec<T>, func: extern "C" fn()) -> svm_sdk::ReturnData
where
    T: svm_sdk::traits::Encoder,
{
    use svm_sdk::host::MockHost;

    let mut bytes = Vec::new();

    for arg in args {
        arg.encode(&mut bytes);
    }

    MockHost::set_raw_calldata(&bytes);

    func();

    let bytes = MockHost::get_returndata();

    svm_sdk::ReturnData::new(&bytes.unwrap())
}

fn main() {
    let mut returns = call(vec![true, true], and);
    let res: bool = returns.next_1();
    assert!(res);

    let mut returns = call(vec![false, true], and);
    let res: bool = returns.next_1();
    assert_eq!(res, false);
}
