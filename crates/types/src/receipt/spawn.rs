use crate::receipt::{CallReceipt, ReceiptLog, RuntimeError};
use crate::{gas::Gas, AccountAddr, State};

/// Returned Receipt after spawning an [`Account`](crate::Account)
#[derive(Debug, PartialEq, Clone)]
pub struct SpawnReceipt {
    /// The transaction format version
    pub version: u16,

    /// whether spawn succeeded or not
    pub success: bool,

    /// the error in case spawning failed
    pub error: Option<RuntimeError>,

    /// the spawned app `Address`
    pub account_addr: Option<AccountAddr>,

    /// the spawned app initial state (after executing its ctor)
    pub init_state: Option<State>,

    /// returned ctor data
    pub returndata: Option<Vec<u8>>,

    /// The amount of gas used
    pub gas_used: Gas,

    /// logged entries during spawn-app's ctor running
    pub logs: Vec<ReceiptLog>,
}

impl SpawnReceipt {
    /// Creates a `SpawnAppReceipt` for reaching reaching `Out-of-Gas`.
    pub fn new_oog(logs: Vec<ReceiptLog>) -> Self {
        Self::from_err(RuntimeError::OOG, logs)
    }

    /// Creates a new failure Receipt out of the `error` parameter
    pub fn from_err(error: RuntimeError, logs: Vec<ReceiptLog>) -> Self {
        Self {
            version: 0,
            success: false,
            error: Some(error),
            account_addr: None,
            init_state: None,
            returndata: None,
            gas_used: Gas::new(),
            logs,
        }
    }

    /// Returns spawned-app `Error`. Panics if spawning has *not* failed.
    pub fn error(&self) -> &RuntimeError {
        self.error.as_ref().unwrap()
    }

    /// Returns spawned-app `Address`. Panics if spawning has failed.
    pub fn app_addr(&self) -> &AccountAddr {
        self.account_addr.as_ref().unwrap()
    }

    /// Returns spawned-app initial `State`. Panics if spawning has failed.
    pub fn init_state(&self) -> &State {
        self.init_state.as_ref().unwrap()
    }

    /// Returns spawned-app results. Panics if spawning has failed.
    pub fn returndata(&self) -> &Vec<u8> {
        self.returndata.as_ref().unwrap()
    }

    /// Returns spawned-app gas-used
    pub fn get_gas_used(&self) -> Gas {
        self.gas_used
    }

    /// Returns the logs generated during the transaction execution
    pub fn logs(&self) -> &[ReceiptLog] {
        &self.logs
    }

    /// Take the Receipt's logged entries out
    pub fn take_logs(&mut self) -> Vec<ReceiptLog> {
        std::mem::take(&mut self.logs)
    }
}

#[allow(missing_docs)]
pub fn into_spawn_receipt(mut ctor_receipt: CallReceipt, app_addr: &AccountAddr) -> SpawnReceipt {
    let logs = ctor_receipt.take_logs();

    if ctor_receipt.success {
        SpawnReceipt {
            version: 0,
            success: true,
            error: None,
            account_addr: Some(app_addr.clone()),
            init_state: ctor_receipt.new_state,
            returndata: ctor_receipt.returndata,
            gas_used: ctor_receipt.gas_used,
            logs,
        }
    } else {
        let error = ctor_receipt.error.unwrap();

        SpawnReceipt {
            version: 0,
            success: false,
            error: Some(error),
            account_addr: None,
            init_state: None,
            returndata: None,
            gas_used: Gas::new(),
            logs,
        }
    }
}
