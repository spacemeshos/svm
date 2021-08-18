use svm_sdk_std::{String, StringBuilder, ToString};

/// Represents a `Layer`
#[derive(PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct LayerId(pub u64);

#[cfg(any(test, feature = "debug"))]
impl core::fmt::Debug for LayerId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LayerId({})", self.0)
    }
}

impl ToString for LayerId {
    fn to_string(&self) -> String {
        let mut sb = StringBuilder::with_capacity(20 + " coins".len());

        let s = self.0.to_string();
        sb.push_str(&String::new_short("[Layer ".as_bytes()));
        sb.push_str(&s);
        sb.push_str(&String::from_byte(b']'));
        sb.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::to_std_string;

    #[test]
    fn layer_to_string() {
        let layer = LayerId(123);
        assert_eq!(to_std_string(layer), "[Layer 123]");
    }
}
