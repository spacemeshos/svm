/// Represent a `Layer` of the Spacemesh Protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Layer(pub u64);

impl Default for Layer {
    fn default() -> Self {
        Self(0)
    }
}
