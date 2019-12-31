use crate::{traits::AppAddressCompute, types::App};

use svm_common::{Address, DefaultKeyHasher, KeyHasher};

pub struct DefaultAppAddressCompute;

impl AppAddressCompute for DefaultAppAddressCompute {
    fn compute(app: &App) -> Address {
        let mut buf = Vec::with_capacity(Address::len() * 2);
        buf.extend_from_slice(app.template.as_slice());
        buf.extend_from_slice(app.creator.as_slice());

        let hash = DefaultKeyHasher::hash(&buf);

        Address::from(&hash[0..Address::len()])
    }
}
