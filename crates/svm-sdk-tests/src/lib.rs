#![allow(unused)]

use svm_sdk;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "mock")] {
        use svm_sdk::storage::MockStorage;
        use svm_sdk::host::MockHost;
    }
    else {
        use svm_sdk::storage::ExtStorage;
        use svm_sdk::host::ExtHost;
    }
}
