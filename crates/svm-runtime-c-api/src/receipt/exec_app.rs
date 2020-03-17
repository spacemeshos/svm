use byteorder::{BigEndian, WriteBytesExt};

use svm_common::State;
use svm_runtime::{
    error::ExecAppError,
    receipt::{ExecReceipt, Receipt},
    value::Value,
};

use super::helpers;
use crate::svm_value_type;

pub(crate) fn encode_exec_receipt(receipt: &ExecReceipt) -> Vec<u8> {
    let mut buf = Vec::new();

    let wrapped_receipt = Receipt::ExecApp(receipt);

    helpers::encode_is_success(&mut buf, &wrapped_receipt);

    if receipt.success {
        encode_new_state(&mut buf, receipt);
        helpers::encode_returns(&mut buf, &wrapped_receipt);
    } else {
        helpers::encode_error(&mut buf, &wrapped_receipt);
    };

    buf
}

fn encode_new_state(buf: &mut Vec<u8>, receipt: &ExecReceipt) {
    debug_assert!(receipt.success);

    let new_state = receipt.get_new_state();

    buf.extend_from_slice(new_state.as_slice());
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{testing, testing::ClientReceipt};

    use svm_common::{Address, State};
    use svm_runtime::{error::ExecAppError, value::Value};

    #[test]
    fn encode_receipt_error() {
        let error = ExecAppError::AppNotFound {
            app_addr: Address::of("my-app").into(),
        };

        let expected = ClientReceipt::Failure {
            error: error.to_string(),
        };

        let receipt = Receipt {
            success: false,
            error: Some(error),
            new_state: None,
            returns: None,
        };

        let bytes = encode_exec_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_receipt_success_without_returns() {
        let new_state = State::from(0x10_20_30_40);

        let expected = ClientReceipt::Success {
            new_state: new_state.clone(),
            func_returns: "".to_string(),
        };

        let receipt = Receipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(Vec::new()),
        };

        let bytes = encode_exec_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_receipt_success_with_returns() {
        let new_state = State::from(0x10_20_30_40);
        let returns = vec![Value::I32(10), Value::I64(20), Value::I32(30)];

        let expected = ClientReceipt::Success {
            new_state: new_state.clone(),
            func_returns: "I32(10), I64(20), I32(30)".to_string(),
        };

        let receipt = Receipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(returns),
        };

        let bytes = encode_exec_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }
}
