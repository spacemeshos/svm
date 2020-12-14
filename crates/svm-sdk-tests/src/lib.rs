#![allow(unused)]

use svm_sdk::host::MockHost;
use svm_sdk::storage::MockStorage;
use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;
use svm_sdk_types::value::Value;

pub fn call<T>(func: extern "C" fn(), args: Vec<T>) -> ReturnData
where
    T: Encoder,
{
    let mut bytes = Vec::new();

    for arg in args {
        arg.encode(&mut bytes);
    }

    MockHost::set_raw_calldata(&bytes);

    func();

    let bytes = MockHost::get_returndata();

    ReturnData::new(&bytes.unwrap())
}

pub fn call_1<T, O>(func: extern "C" fn(), args: Vec<T>) -> O
where
    T: Encoder,
    O: From<Value<'static>>,
{
    let mut returns = call(func, args);

    returns.next_1()
}

pub fn call_2<T, O1, O2>(func: extern "C" fn(), args: Vec<T>) -> (O1, O2)
where
    T: Encoder,
    O1: From<Value<'static>>,
    O2: From<Value<'static>>,
{
    let mut returns = call(func, args);

    returns.next_2()
}

pub fn call_3<T, O1, O2, O3>(func: extern "C" fn(), args: Vec<T>) -> (O1, O2, O3)
where
    T: Encoder,
    O1: From<Value<'static>>,
    O2: From<Value<'static>>,
    O3: From<Value<'static>>,
{
    let mut returns = call(func, args);

    returns.next_3()
}

pub fn call_4<T, O1, O2, O3, O4>(func: extern "C" fn(), args: Vec<T>) -> (O1, O2, O3, O4)
where
    T: Encoder,
    O1: From<Value<'static>>,
    O2: From<Value<'static>>,
    O3: From<Value<'static>>,
    O4: From<Value<'static>>,
{
    let mut returns = call(func, args);

    returns.next_4()
}
