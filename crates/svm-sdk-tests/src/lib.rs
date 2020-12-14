#![allow(unused)]

use svm_sdk::host::MockHost;
use svm_sdk::storage::MockStorage;
use svm_sdk::traits::Encoder;
use svm_sdk::{Amount, ReturnData};
use svm_sdk_types::value::Value;

pub fn call<T>(func: extern "C" fn(), args: Vec<T>) -> ReturnData
where
    T: Encoder,
{
    call_and_fund(func, args, Amount(0))
}

pub fn call_and_fund<T>(func: extern "C" fn(), args: Vec<T>, value: Amount) -> ReturnData
where
    T: Encoder,
{
    let mut bytes = Vec::new();

    for arg in args {
        arg.encode(&mut bytes);
    }

    MockHost::set_raw_calldata(&bytes);
    MockHost::set_value(value);

    // TODO: MockHost::transfer(from: sender, to: app)

    func();

    let bytes = MockHost::get_returndata();

    ReturnData::new(&bytes.unwrap())
}

pub fn call_1<T, O>(func: extern "C" fn(), args: Vec<T>) -> O
where
    T: Encoder,
    O: From<Value<'static>>,
{
    call_and_fund_1(func, args, Amount(0))
}

pub fn call_and_fund_1<T, O>(func: extern "C" fn(), args: Vec<T>, value: Amount) -> O
where
    T: Encoder,
    O: From<Value<'static>>,
{
    let mut returns = call_and_fund(func, args, value);

    returns.next_1()
}

pub fn call_2<T, O1, O2>(func: extern "C" fn(), args: Vec<T>) -> (O1, O2)
where
    T: Encoder,
    O1: From<Value<'static>>,
    O2: From<Value<'static>>,
{
    call_and_fund_2(func, args, Amount(0))
}

pub fn call_and_fund_2<T, O1, O2>(func: extern "C" fn(), args: Vec<T>, value: Amount) -> (O1, O2)
where
    T: Encoder,
    O1: From<Value<'static>>,
    O2: From<Value<'static>>,
{
    let mut returns = call_and_fund(func, args, value);

    returns.next_2()
}
