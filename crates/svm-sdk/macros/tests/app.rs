#![allow(unused)]

use svm_sdk::{Address, Amount};
use svm_sdk_macros::app;

#[app]
mod MyApp {
    fn add(a: Amount, b: Amount) -> Amount {
        a + b
    }
}
