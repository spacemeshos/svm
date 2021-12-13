use svm_types::Gas;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GasTank {
    NonEmpty(u64),
    Empty,
}

impl GasTank {
    pub fn new(gas: Gas) -> Self {
        let gas = gas.unwrap_or(std::u64::MAX);

        if gas > 0 {
            GasTank::NonEmpty(gas)
        } else {
            GasTank::Empty
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn consume(self, gas: u64) -> GasTank {
        match self {
            GasTank::Empty => GasTank::Empty,
            GasTank::NonEmpty(left) => {
                if left > gas {
                    GasTank::NonEmpty(left - gas)
                } else {
                    GasTank::Empty
                }
            }
        }
    }

    pub fn unwrap(self) -> u64 {
        match self {
            GasTank::Empty => 0,
            GasTank::NonEmpty(gas) => gas,
        }
    }
}
