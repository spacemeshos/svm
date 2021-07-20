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

impl Default for Envelope {
    fn default() -> Self {
        Self::with_principal(Address::zeros())
    }
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

    /// Creates a new [`Envelope`] with the given `principal` parameter.
    ///
    /// Sets default values for all remaining fields.
    /// Sets no `gas limit` (suitable when running with gas pricing off).
    ///
    /// # Notes
    ///
    /// This method should be useful to ease tests setup.
    pub fn with_principal(principal: Address) -> Self {
        Self {
            principal,
            amount: 0,
            gas_limit: Gas::new(),
            gas_fee: 0,
        }
    }

    /// Creates a new [`Envelope`] with the given `gas_limit` parameter.
    ///
    /// Sets default values for all remaining fields.
    ///
    /// # Notes
    ///
    /// This method should be useful to ease tests setup.
    pub fn with_gas_limit(gas_limit: Gas) -> Self {
        Self {
            principal: Address::zeros(),
            amount: 0,
            gas_limit,
            gas_fee: 0,
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
