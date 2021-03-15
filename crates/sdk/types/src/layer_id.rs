#[derive(PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct LayerId(pub u64);

#[cfg(any(test, feature = "debug"))]
impl core::fmt::Debug for LayerId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LayerId({})", self.0)
    }
}
