//!  ## `Deploy Template` Receipt Binary Format Version 0
//!
//!  On success (`is_success = 1`)
//!
//!  ```text
//!  +-----------------------------------------------------------------------+
//!  |          |             |             |                    |           |
//!  | tx type  |   version   |  is_success | template `Address` | gas_used  |
//!  | (1 byte) |  (2 bytes)  |  (1 byte)   |     (20 bytes)     | (8 bytes) |
//!  |          |             |             |                    |           |
//!  +-----------------------------------------------------------------------+
//!  ```
//!
//!  On Error (`is_success = 0`)
//!  See [error.rs][./error.rs]

use svm_types::{DeployReceipt, Gas, ReceiptLog, TemplateAddr};

use super::{error::RuntimeErrorWithLogs, TY_DEPLOY};
use crate::{version, Codec};
use crate::{ReadExt, WriteExt};

impl Codec for DeployReceipt {
    type Error = std::convert::Infallible;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(TY_DEPLOY);
        version::encode_version(self.version, w);
        self.success.encode(w);

        if self.success {
            self.template_addr().0.encode(w);
            self.gas_used.encode(w);
            self.logs.encode(w);
        } else {
            RuntimeErrorWithLogs::new(self.error().clone(), vec![]).encode(w);
        };
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let ty = cursor.read_byte().unwrap();
        debug_assert_eq!(ty, TY_DEPLOY);

        let version = version::decode_version(cursor).unwrap();
        debug_assert_eq!(version, 0);

        let is_success = bool::decode(cursor).unwrap();

        if is_success {
            let addr = TemplateAddr::decode(cursor).expect("expected a Template Address");
            let gas_used = Gas::decode(cursor).unwrap();
            let logs = <Vec<ReceiptLog>>::decode(cursor).unwrap();

            Ok(DeployReceipt {
                version,
                success: true,
                error: None,
                addr: Some(addr),
                gas_used,
                logs,
            })
        } else {
            let x = RuntimeErrorWithLogs::decode(cursor).unwrap();
            Ok(DeployReceipt::from_err(x.err, x.logs))
        }
    }
}

#[cfg(test)]
mod tests {
    use svm_types::{BytesPrimitive, DeployReceipt, Gas, Receipt, TemplateAddr};

    use crate::Codec;

    #[test]
    fn encode_decode_deploy_template_receipt() {
        let addr = TemplateAddr::repeat(0xAB);

        let receipt = DeployReceipt {
            version: 0,
            success: true,
            error: None,
            addr: Some(addr),
            gas_used: Gas::with(100),
            logs: Vec::new(),
        };

        let bytes = receipt.encode_to_vec();
        let decoded = Receipt::decode_bytes(bytes).unwrap();

        assert_eq!(decoded.into_deploy(), receipt);
    }
}
