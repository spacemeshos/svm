//! Encoding of a binary [`Envelope`].
//!
//! ```text
//!
//!  +-------------+--------------+----------------+----------------+
//!  |             |              |                |                |
//!  |  Principal  |    Amount    |   Gas Limit    |    Gas Fee     |
//!  |  (Address)  |    (u64)     |     (u64)      |     (u64)      |
//!  |             |              |                |                |
//!  |  20 bytes   |   8 bytes    |    8 bytes     |    8 bytes     |
//!  |             | (Big-Endian) |  (Big-Endian)  |  (Big-Endian)  |
//!  |             |              |                |                |
//!  +-------------+--------------+----------------+----------------+
//!
//! ```

use std::io::Cursor;

use svm_types::{Envelope, Gas};

use crate::{ReadExt, WriteExt};

/// Returns the number of bytes required to hold a binary [`Envelope`].
pub const fn byte_size() -> usize {
    20 + 8 + 8 + 8
}

/// Encodes a binary [`Envelope`] of a transaction.
pub fn encode(envelope: &Envelope, w: &mut Vec<u8>) {
    w.write_address(envelope.principal());
    w.write_u64_be(envelope.amount());
    w.write_u64_be(envelope.gas_limit().unwrap_or(0));
    w.write_u64_be(envelope.gas_fee());
}

/// Decodes a binary [`Envelope`] of a transaction.
///
/// Returns the decoded [`Envelope`],
/// On failure, returns [`std::io::Result`].
pub fn decode(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Envelope> {
    let principal = cursor.read_address()?;
    let amount = cursor.read_u64_be()?;
    let gas_limit = cursor.read_u64_be()?;
    let gas_fee = cursor.read_u64_be()?;

    let gas_limit = if gas_limit > 0 {
        Gas::with(gas_limit)
    } else {
        Gas::new()
    };

    let envelope = Envelope::new(principal, amount, gas_limit, gas_fee);
    Ok(envelope)
}
