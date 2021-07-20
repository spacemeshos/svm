use crate::{Address, Gas};

/// Holds `Transaction` **agnostic** content.
///
/// Once created it should NOT be modified (immutable).
pub struct Envelope {
    principal: Address,
    amount: u64,
    gas_limit: Gas,
    gas_fee: u64,
}

impl Envelope {
    /// Creates a new [`Envelope`].
    pub fn new(principal: Address, amount: u64, gas_limit: Gas, gas_fee: u64) -> Self {
        Self {
            principal,
            amount,
            gas_limit,
            gas_fee,
        }
    }

    /// The `Address` of the `Account` paying for the [`Gas`].
    pub fn principal(&self) -> &Address {
        &self.principal
    }

    /// Funding by the `Principal`.
    pub fn amount(&self) -> u64 {
        self.amount
    }

    /// Maximum units of Gas to be paid.
    pub fn gas_limit(&self) -> Gas {
        self.gas_limit
    }

    /// Fee per Unit of [`Gas`].
    pub fn gas_fee(&self) -> u64 {
        self.gas_fee
    }
}
