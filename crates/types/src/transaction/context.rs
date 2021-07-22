use crate::{Layer, State, TransactionId};

/// Holds the `Transaction`'s Context.
///
/// Once created it **can't** be modified (it's immutable).
///
/// It contains properties related to the context in which a Transaction will execute.
/// Additionally, it will encapsulate properties inferred from the binary Transaction sent over-the-wire.
///
/// The [`Context`]'s data should be passed externally from the `Node`.
/// That's why we are not allowed to touch its content, and have it immutable.
#[derive(Debug, Clone)]
pub struct Context {
    tx_id: TransactionId,
    layer: Layer,
    state: State,
}

impl Default for Context {
    fn default() -> Self {
        Self::with_state(State::zeros())
    }
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

    /// Creates a new [`Context`] with the given `state` parameter.
    ///
    /// Sets default values for all remaining fields.
    ///
    /// # Notes
    ///
    /// This method should be useful to ease tests setup.
    pub fn with_state(state: State) -> Self {
        Self {
            tx_id: TransactionId::zeros(),
            layer: Layer::default(),
            state: state,
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
