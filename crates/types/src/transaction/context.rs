use crate::{Layer, State, TransactionId};

/// Holds the a `Transaction`'s Context.
///
/// Once created it should NOT be modified (immutable).
/// It contains properties related to the context in which a Transaction will execute.
/// Additionally, it will encapsulate properties inferred from the binary Transaction sent over-the-wire.
#[derive(Debug, Clone)]
pub struct Context {
    tx_id: TransactionId,
    layer: Layer,
    state: State,
}

impl Context {
    /// Creates a new [`Context`].
    pub fn new(tx_id: TransactionId, layer: Layer, state: State) -> Self {
        Self {
            tx_id,
            layer,
            state,
        }
    }

    /// The [`TransactionId`]
    pub fn tx_id(&self) -> &TransactionId {
        &self.tx_id
    }

    /// The current [`Layer`].
    pub fn layer(&self) -> Layer {
        self.layer
    }

    /// The current Root Hash `State`.
    pub fn state(&self) -> &State {
        &self.state
    }
}
