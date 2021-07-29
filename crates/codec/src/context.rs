//! Encoding of a binary [`Context`].
//!
//! ```text
//!
//!  +------------------+-----------------+-----------------+
//!  |                  |                 |                 |
//!  |  Transaction Id  |  Current Layer  |  Current State  |
//!  |     (Hash)       |     (u64)       |     (State)     |
//!  |                  |                 |                 |
//!  |    32 bytes      |    8 bytes      |    32 bytes     |
//!  |                  |   (Big-Endian)  |                 |
//!  |                  |                 |                 |
//!  +------------------+-----------------+-----------------+
//!
//! ```

use std::io::Cursor;

use svm_types::{Context, Layer};

use crate::{ReadExt, WriteExt};

/// Returns the number of bytes required to hold a binary [`Context`].
pub const fn byte_size() -> usize {
    32 + 8 + 32
}

/// Encodes a binary [`Context`] of a transaction.
pub fn encode(context: &Context, w: &mut Vec<u8>) {
    w.write_tx_id(context.tx_id());
    w.write_u64_be(context.layer().0);
    w.write_state(context.state());
}

/// Decodes a binary [`Context`] of a transaction.
///
/// Returns the decoded [`Context`],
/// On failure, returns [`std::io::Result`].
pub fn decode(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Context> {
    let tx_id = cursor.read_tx_id()?;
    let layer = cursor.read_u64_be()?;
    let state = cursor.read_state()?;

    let context = Context::new(tx_id, Layer(layer), state);
    Ok(context)
}
