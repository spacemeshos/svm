use crate::{Layer, State, TransactionId};

/// Holds the a `Transaction`'s Context.
///
/// It contains properties related to the context in which a Transaction will execute.
/// Additionally, it will encapsulate properties inferred from the binary Transaction sent over-the-wire.
#[derive(Debug, Clone)]
pub struct Context {
    /// The `Transaction` Id
    pub tx_id: TransactionId,

    /// The current [`Layer`].
    pub layer: Layer,

    /// The current Root Hash `State`.
    pub state: State,
}
