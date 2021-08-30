use derive_more::{AsRef, From};

use crate::BytesPrimitive;

/// The unique identifier of a [`Transaction`](crate::Transaction).
#[derive(Debug, Copy, Clone, From, Hash, PartialEq, Eq, AsRef)]
pub struct TransactionId(pub [u8; 32]);

impl BytesPrimitive<32> for TransactionId {}
