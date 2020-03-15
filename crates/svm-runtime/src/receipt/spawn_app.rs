use svm_app::types::AppAddr;

use super::Receipt;

use crate::{error::SpawnAppError, value::Value};

use svm_common::State;

#[derive(Debug)]
pub struct SpawnAppReceipt {
    /// whether spawn succedded or not
    pub success: bool,

    /// the error in case spawning failed
    pub error: Option<SpawnAppError>,

    /// the spawned app `Address`
    pub addr: Option<AppAddr>,

    /// the spawned app initial state (after executing its ctor)
    pub init_state: Option<State>,

    /// returned ctor values
    pub returns: Option<Vec<Value>>,
}

impl From<&SpawnAppError> for SpawnAppReceipt {
    fn from(error: &SpawnAppError) -> Self {
        Self {
            success: false,
            error: Some(error.clone()),
            addr: None,
            init_state: None,
            returns: None,
        }
    }
}

pub fn make_spawn_app_receipt(ctor_receipt: Receipt, addr: &AppAddr) -> SpawnAppReceipt {
    if ctor_receipt.success {
        SpawnAppReceipt {
            success: true,
            error: None,
            addr: None,
            init_state: ctor_receipt.new_state,
            returns: ctor_receipt.returns,
        }
    } else {
        let error = ctor_receipt.error.unwrap();

        SpawnAppReceipt {
            success: false,
            error: Some(SpawnAppError::CtorFailed(error)),
            addr: Some(addr.clone()),
            init_state: None,
            returns: None,
        }
    }
}
