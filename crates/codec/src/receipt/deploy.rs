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

use std::convert::TryFrom;

use svm_types::{DeployReceipt, Gas, ReceiptLog, RuntimeFailure, TemplateAddr};

use super::TY_DEPLOY;
use crate::{Codec, ReadExt, WriteExt};

impl Codec for DeployReceipt {
    type Error = std::convert::Infallible;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(TY_DEPLOY);
        u16::try_from(self.version).unwrap().encode(w);
        self.success.encode(w);

        if self.success {
            self.template_addr().0.encode(w);
            self.gas_used.encode(w);
            self.logs.encode(w);
        } else {
            RuntimeFailure::new(self.error().clone(), vec![]).encode(w);
        };
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let ty = reader.read_byte().unwrap();
        debug_assert_eq!(ty, TY_DEPLOY);

        let version = u16::decode(reader).unwrap();
        debug_assert_eq!(version, 0);

        let is_success = bool::decode(reader).unwrap();

        if is_success {
            let addr = TemplateAddr::decode(reader).expect("expected a Template Address");
            let gas_used = Gas::decode(reader).unwrap();
            let logs = <Vec<ReceiptLog>>::decode(reader).unwrap();

            Ok(DeployReceipt {
                version,
                success: true,
                error: None,
                addr: Some(addr),
                gas_used,
                logs,
            })
        } else {
            let x = RuntimeFailure::decode(reader).unwrap();
            Ok(DeployReceipt::from_err(x.err, x.logs))
        }
    }
}

#[cfg(test)]
mod tests {
    use svm_types::{BytesPrimitive, DeployReceipt, Gas, TemplateAddr};

    use crate::codec::test_codec;

    #[test]
    fn encode_decode_deploy_template_receipt() {
        let addr = TemplateAddr::repeat(0xAB);

        test_codec(DeployReceipt {
            version: 0,
            success: true,
            error: None,
            addr: Some(addr),
            gas_used: Gas::with(100),
            logs: Vec::new(),
        });
    }
}
