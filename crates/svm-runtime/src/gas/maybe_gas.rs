use std::ops::Sub;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct MaybeGas(Option<u64>);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OOGError {}

impl MaybeGas {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn with(gas: u64) -> Self {
        Self(Some(gas))
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    #[inline]
    pub fn unwrap(&self) -> u64 {
        self.0.unwrap()
    }

    #[inline]
    pub fn unwrap_or(&self, default: u64) -> u64 {
        self.0.unwrap_or(default)
    }
}

impl Sub<u64> for MaybeGas {
    type Output = Result<MaybeGas, OOGError>;

    fn sub(self, rhs: u64) -> Self::Output {
        match (self.0, rhs) {
            (None, _) => Ok(MaybeGas::new()),
            (Some(lhs), rhs) => {
                if lhs >= rhs {
                    let maybe_gas = MaybeGas::with(lhs - rhs);

                    Ok(maybe_gas)
                } else {
                    Err(OOGError {})
                }
            }
        }
    }
}

impl From<u64> for MaybeGas {
    fn from(v: u64) -> Self {
        MaybeGas::with(v)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn maybe_gas_sub() {
        todo!()
    }
}
