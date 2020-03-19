use super::ExecReceipt;

use crate::error::SpawnAppError;

use svm_app::types::{AppAddr, WasmValue};
use svm_common::State;

/// Returned Receipt after spawning an App.
#[derive(Debug)]
pub struct SpawnAppReceipt {
    /// whether spawn succedded or not
    pub success: bool,

    /// the error in case spawning failed
    pub error: Option<SpawnAppError>,

    /// the spawned app `Address`
    pub app_addr: Option<AppAddr>,

    /// the spawned app initial state (after executing its ctor)
    pub init_state: Option<State>,

    /// returned ctor values
    pub returns: Option<Vec<WasmValue>>,

    /// The amount of gas used
    pub gas_used: Option<u64>,
}

impl SpawnAppReceipt {
    /// Returns spawned-app `Address`. Panics if spawning has failed.
    pub fn get_app_addr(&self) -> &AppAddr {
        self.app_addr.as_ref().unwrap()
    }

    /// Returns spawned-app initial `State`. Panics if spawning has failed.
    pub fn get_init_state(&self) -> &State {
        self.init_state.as_ref().unwrap()
    }

    /// Returns spawned-app results. Panics if spawning has failed.
    pub fn get_returns(&self) -> &Vec<WasmValue> {
        self.returns.as_ref().unwrap()
    }
}

impl From<SpawnAppError> for SpawnAppReceipt {
    fn from(error: SpawnAppError) -> Self {
        Self {
            success: false,
            error: Some(error),
            app_addr: None,
            init_state: None,
            returns: None,
            gas_used: None,
        }
    }
}

#[allow(missing_docs)]
pub fn make_spawn_app_receipt(ctor_receipt: ExecReceipt, app_addr: &AppAddr) -> SpawnAppReceipt {
    let app_addr = Some(app_addr.clone());

    if ctor_receipt.success {
        SpawnAppReceipt {
            success: true,
            error: None,
            app_addr,
            init_state: ctor_receipt.new_state,
            returns: ctor_receipt.returns,
            gas_used: ctor_receipt.gas_used,
        }
    } else {
        let error = ctor_receipt.error.unwrap();

        SpawnAppReceipt {
            success: false,
            error: Some(SpawnAppError::CtorFailed(error)),
            app_addr,
            init_state: None,
            returns: None,
            gas_used: None,
        }
    }
}
