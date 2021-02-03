use std::io::SeekFrom;

use crate::receipt::{ExecReceipt, Log, ReceiptError};
use crate::{gas::MaybeGas, AppAddr, State};

/// Returned Receipt after spawning an App.
#[derive(Debug, PartialEq, Clone)]
pub struct SpawnAppReceipt {
    pub version: u16,

    /// whether spawn succedded or not
    pub success: bool,

    /// the error in case spawning failed
    pub error: Option<ReceiptError>,

    /// the spawned app `Address`
    pub app_addr: Option<AppAddr>,

    /// the spawned app initial state (after executing its ctor)
    pub init_state: Option<State>,

    /// returned ctor data
    pub returndata: Option<Vec<u8>>,

    /// The amount of gas used
    pub gas_used: MaybeGas,

    /// logged entries during spawn-app's ctor running
    pub logs: Vec<Log>,
}

impl SpawnAppReceipt {
    /// Creates a `SpawnAppReceipt` for reaching reaching `Out-of-Gas`.
    pub fn new_oog(logs: Vec<Log>) -> Self {
        Self::from_err(ReceiptError::OOG, Vec::new())
    }

    pub fn from_err(error: ReceiptError, logs: Vec<Log>) -> Self {
        Self {
            version: 0,
            success: false,
            error: Some(error),
            app_addr: None,
            init_state: None,
            returndata: None,
            gas_used: MaybeGas::new(),
            logs,
        }
    }

    /// Returns spawned-app `Error`. Panics if spawning has *not* failed.
    pub fn get_error(&self) -> &ReceiptError {
        self.error.as_ref().unwrap()
    }

    /// Returns spawned-app `Address`. Panics if spawning has failed.
    pub fn get_app_addr(&self) -> &AppAddr {
        self.app_addr.as_ref().unwrap()
    }

    /// Returns spawned-app initial `State`. Panics if spawning has failed.
    pub fn get_init_state(&self) -> &State {
        self.init_state.as_ref().unwrap()
    }

    /// Returns spawned-app results. Panics if spawning has failed.
    pub fn get_returndata(&self) -> &Vec<u8> {
        self.returndata.as_ref().unwrap()
    }

    /// Returns spawned-app gas-used
    pub fn get_gas_used(&self) -> MaybeGas {
        self.gas_used
    }

    pub fn get_logs(&self) -> &[Log] {
        &self.logs
    }

    pub fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }
}

#[allow(missing_docs)]
pub fn into_spawn_app_receipt(
    mut ctor_receipt: ExecReceipt,
    app_addr: &AppAddr,
) -> SpawnAppReceipt {
    let app_addr = Some(app_addr.clone());
    let logs = ctor_receipt.take_logs();

    if ctor_receipt.success {
        SpawnAppReceipt {
            version: 0,
            success: true,
            error: None,
            app_addr,
            init_state: ctor_receipt.new_state,
            returndata: ctor_receipt.returndata,
            gas_used: ctor_receipt.gas_used,
            logs,
        }
    } else {
        let error = ctor_receipt.error.unwrap();

        SpawnAppReceipt {
            version: 0,
            success: false,
            error: Some(error),
            app_addr,
            init_state: None,
            returndata: None,
            gas_used: MaybeGas::new(),
            logs,
        }
    }
}
