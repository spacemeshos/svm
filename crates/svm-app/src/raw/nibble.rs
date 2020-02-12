#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Nibble(u8);

#[macro_export]
macro_rules! nib {
    ($val:expr) => {{
        Nibble::new($val)
    }};
}

impl Nibble {
    #[inline]
    pub fn new(byte: u8) -> Self {
        assert!(byte <= 0x0F);

        Self(byte)
    }

    #[inline]
    pub fn inner(&self) -> u8 {
        self.0
    }

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

pub fn concat_nibbles(nibs: &[Nibble]) -> (Vec<u8>, Option<Nibble>) {
    let cap = nibs.len() / 2 + 1;
    let mut bytes = Vec::with_capacity(cap);
    let mut iter = nibs.chunks_exact(2);

    while let Some(chunk) = iter.next() {
        let (lnib, rnib) = (chunk[0], chunk[1]);

        let byte = lnib.0 << 4 | rnib.0;
        bytes.push(byte);
    }

    let rem = iter.remainder();

    if rem.len() > 0 {
        assert_eq!(1, rem.len());

        let rem_nib = rem[0];
        (bytes, Some(rem_nib))
    } else {
        (bytes, None)
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

    #[test]
    fn concat_nibbles_even_nibbles() {
        let nib1 = Nibble(0b_0000_1001);
        let nib2 = Nibble(0b_0000_0110);
        let nib3 = Nibble(0b_0000_1100);
        let nib4 = Nibble(0b_0000_0011);

        assert_eq!((vec![], None), concat_nibbles(&[]));
        assert_eq!((vec![0b_1001_0110], None), concat_nibbles(&[nib1, nib2]));

        assert_eq!(
            (vec![0b_1001_0110, 0b_1100_0011], None),
            concat_nibbles(&[nib1, nib2, nib3, nib4])
        );
    }

    #[test]
    fn concat_nibbles_odd_nibbles() {
        let nib1 = Nibble(0b_0000_1001);
        let nib2 = Nibble(0b_0000_0110);
        let nib3 = Nibble(0b_0000_1100);
        let nib4 = Nibble(0b_0000_0011);
        let nib5 = Nibble(0b_0000_1010);

        assert_eq!((vec![], Some(nib1)), concat_nibbles(&[nib1]));
        assert_eq!(
            (vec![0b_1001_0110], Some(nib3)),
            concat_nibbles(&[nib1, nib2, nib3])
        );

        assert_eq!(
            (vec![0b_1001_0110, 0b_1100_0011], Some(nib5)),
            concat_nibbles(&[nib1, nib2, nib3, nib4, nib5])
        );
    }
}
