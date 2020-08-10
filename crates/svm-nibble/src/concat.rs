use crate::Nibble;

/// Concatenates a slice of `Nibble`(s) into a `Vec<u8>`.
/// Two consecutive nibbles become a single byte.
/// In case the `nibs.len()` is odd, returns the remainder `Nibble` too.
pub fn concat_nibbles(nibs: &[Nibble]) -> (Vec<u8>, Option<Nibble>) {
    let cap = nibs.len() / 2 + 1;
    let mut bytes = Vec::with_capacity(cap);
    let mut iter = nibs.chunks_exact(2);

    while let Some(chunk) = iter.next() {
        let (lnib, rnib) = (chunk[0], chunk[1]);

        let byte = lnib.inner() << 4 | rnib.inner();
        bytes.push(byte);
    }

    let rem = iter.remainder();

    if !rem.is_empty() {
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

    use crate::nib;

    #[test]
    fn concat_nibbles_even_nibbles() {
        let nib1 = nib!(0b_0000_1001);
        let nib2 = nib!(0b_0000_0110);
        let nib3 = nib!(0b_0000_1100);
        let nib4 = nib!(0b_0000_0011);

        assert_eq!((vec![], None), concat_nibbles(&[]));
        assert_eq!((vec![0b_1001_0110], None), concat_nibbles(&[nib1, nib2]));

        assert_eq!(
            (vec![0b_1001_0110, 0b_1100_0011], None),
            concat_nibbles(&[nib1, nib2, nib3, nib4])
        );
    }

    #[test]
    fn concat_nibbles_odd_nibbles() {
        let nib1 = nib!(0b_0000_1001);
        let nib2 = nib!(0b_0000_0110);
        let nib3 = nib!(0b_0000_1100);
        let nib4 = nib!(0b_0000_0011);
        let nib5 = nib!(0b_0000_1010);

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
