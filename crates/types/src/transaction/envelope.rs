use crate::{Address, Gas};

/// Holds `Transaction` **agnostic** content.
pub struct Envelope {
    /// The `Address` of the `Account` paying for the [`Gas`].
    pub principal: Address,

    /// Funding by the `Principal`.
    pub amount: u64,

    /// Maximum units of Gas to be paid.
    pub gas_limit: Gas,

    /// Fee per Unit of [`Gas`].
    pub gas_fee: u64,
}
