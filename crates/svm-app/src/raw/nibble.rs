#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Nibble(pub u8);

impl Nibble {
    #[inline]
    pub fn is_msb_on(&self) -> bool {
        let msb = self.0 & 0b_0000_1000;
        msb != 0
    }

    #[inline]
    pub fn is_msb_off(&self) -> bool {
        !self.is_msb_on()
    }

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
        let nib = Nibble(0b_1000_0000);
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
