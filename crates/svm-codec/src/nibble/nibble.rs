/// Represents a Nibble (4 bits).
///
/// The nibble is represented by a single byte.
/// When the 4 MSB-bits are zeros.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Nibble(u8);

/// Creates a new `Nibble`.
#[macro_export]
macro_rules! nib {
    ($val:expr) => {{
        use crate::Nibble;

        Nibble::new($val)
    }};
}

impl Nibble {
    /// Creates a new Nibble.
    /// asserts that input `byte` is less than or equal to 0x0F (binary: 000_1111).
    #[inline]
    pub fn new(byte: u8) -> Self {
        assert!(byte <= 0x0F);

        Self(byte)
    }

    /// Returns the underlying byte.
    #[inline]
    pub fn inner(&self) -> u8 {
        self.0
    }

    /// Returns whether the nibble MSB bit is on.
    ///
    /// # Example
    ///
    /// ```rust
    /// use svm_app::raw::Nibble;
    ///
    /// let nib = Nibble::new(0b_0000_1000);
    /// assert!(nib.is_msb_on());
    /// ```
    #[inline]
    pub fn is_msb_on(&self) -> bool {
        let msb = self.0 & 0b_0000_1000;
        msb != 0
    }

    /// Returns whether the nibble MSB bit is off.
    ///
    /// # Example
    ///
    /// ```rust
    /// use svm_app::raw::Nibble;
    ///
    /// let nib = Nibble::new(0b_0000_0001);
    /// assert!(nib.is_msb_off());
    /// ```
    #[inline]
    pub fn is_msb_off(&self) -> bool {
        !self.is_msb_on()
    }

    /// Returns the underlying Nibble as a 4-bool array.
    ///
    /// # Example
    ///
    /// ```rust
    /// use svm_app::raw::Nibble;
    ///
    /// let nib = Nibble::new(0b_0000_1001);
    /// assert_eq!([true, false, false, true], nib.bits());
    /// ```
    pub fn bits(&self) -> [bool; 4] {
        let msb_0 = self.0 & 0b_0000_1000 != 0;
        let msb_1 = self.0 & 0b_0000_0100 != 0;
        let msb_2 = self.0 & 0b_0000_0010 != 0;
        let msb_3 = self.0 & 0b_0000_0001 != 0;

        [msb_0, msb_1, msb_2, msb_3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nibble_is_msb_on_or_off() {
        let nib = Nibble(0b_0000_1000);
        assert!(nib.is_msb_on());
        assert!(!nib.is_msb_off());

        let nib = Nibble(0b_0000_0000);
        assert!(nib.is_msb_off());
        assert!(!nib.is_msb_on());
    }

    #[test]
    fn nibble_bits() {
        let nib = Nibble(0b_0000_1101);
        assert_eq!([true, true, false, true], nib.bits());

        let nib = Nibble(0b_0000_0010);
        assert_eq!([false, false, true, false], nib.bits());
    }
}
