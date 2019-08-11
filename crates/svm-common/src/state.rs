use crate::impl_bytes_primitive;

impl_bytes_primitive!(State, 32);

impl State {
    /// Returns an empty state
    pub fn empty() -> State {
        State([0; 32])
    }
}

impl From<&[u8]> for State {
    fn from(slice: &[u8]) -> State {
        assert_eq!(
            32,
            slice.len(),
            "`State::from` expects exactly 32 bytes input"
        );

        let mut bytes = [0; 32];
        bytes.copy_from_slice(slice);

        State(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!([0; 32], State::empty().0);
    }

    #[test]
    #[should_panic(expected = "`State::from` expects exactly 32 bytes input")]
    fn pages_state_expects_exactly_32_bytes_input() {
        State::from([0; 10].as_ref());
    }

    #[test]
    fn pages_state_from_slice() {
        let raw: [u8; 32] = [
            01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 20, 30, 40, 50, 60, 70, 80, 90, 11, 22, 33, 44,
            55, 66, 77, 88, 99, 251, 252, 253, 254, 255,
        ];

        let state = State::from(raw.as_ref());

        assert_eq!(
            State([
                01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 20, 30, 40, 50, 60, 70, 80, 90, 11, 22, 33,
                44, 55, 66, 77, 88, 99, 251, 252, 253, 254, 255
            ]),
            state
        );
    }
}
