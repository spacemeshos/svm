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

use svm_types::{DeployReceipt, Gas};

use super::{decode_error, encode_error, logs, TY_DEPLOY};
use crate::{version, Codec};
use crate::{ReadExt, WriteExt};

impl Codec for DeployReceipt {
    type Error = std::convert::Infallible;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(TY_DEPLOY);
        version::encode_version(self.version, w);
        w.write_bool(self.success);

        if self.success {
            w.write_bytes_prim(self.template_addr());
            self.gas_used.encode(w);
            logs::encode_logs(&self.logs, w);
        } else {
            let logs = Vec::new();

            encode_error(self.error(), &logs, w);
        };
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let ty = cursor.read_byte().unwrap();
        debug_assert_eq!(ty, TY_DEPLOY);

        let version = version::decode_version(cursor).unwrap();
        debug_assert_eq!(version, 0);

        let is_success = cursor.read_bool().unwrap();

        match is_success {
            false => {
                let (err, logs) = decode_error(cursor);

                Ok(DeployReceipt::from_err(err, logs))
            }
            true => {
                let addr = cursor
                    .read_bytes_prim()
                    .expect("expected a Template Address");
                let gas_used = Gas::decode(cursor).unwrap();
                let logs = logs::decode_logs(cursor).unwrap();

                Ok(DeployReceipt {
                    version,
                    success: true,
                    error: None,
                    addr: Some(addr),
                    gas_used,
                    logs,
                })
            }
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
