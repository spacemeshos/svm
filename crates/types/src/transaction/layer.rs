use derive_more::{Add, From, Into};

/// Represent a `Layer` of the Spacemesh Protocol
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into, Add)]
#[repr(transparent)]
pub struct Layer(pub u64);
