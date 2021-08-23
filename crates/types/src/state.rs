use derive_more::{AsRef, From};

use crate::BytesPrimitive;

/// The fingerprint of a generic state of things.
#[derive(Debug, Copy, Clone, From, Hash, PartialEq, Eq, AsRef)]
pub struct State(pub [u8; 32]);

impl AsRef<[u8]> for State {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl BytesPrimitive<32> for State {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_zeros() {
        assert_eq!([0; 32], State::zeros().0);

        assert!(State::zeros().is_zeros());
    }

    #[test]
    #[should_panic]
    fn state_expects_exactly_32_bytes_input() {
        State::new([0; 10]);
    }

    #[test]
    fn state_from_slice() {
        let raw: [u8; 32] = [
            01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 20, 30, 40, 50, 60, 70, 80, 90, 11, 22, 33, 44,
            55, 66, 77, 88, 99, 251, 252, 253, 254, 255,
        ];

        let state = State::new(raw);

        assert_eq!(
            State([
                01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 20, 30, 40, 50, 60, 70, 80, 90, 11, 22, 33,
                44, 55, 66, 77, 88, 99, 251, 252, 253, 254, 255
            ]),
            state
        );
    }
}
