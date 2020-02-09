use std::{
    io::{Cursor, Read},
    iter::Iterator,
};

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

pub struct NibbleIter<'a> {
    buf: [u8; 1],
    length: u64,
    no_more_bytes: bool,
    last_byte: Option<u8>,
    cursor: Cursor<&'a [u8]>,
}

impl<'a> NibbleIter<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let cursor = Cursor::new(data);
        let length = cursor.get_ref().len() as u64;

        Self {
            cursor,
            length,
            buf: [0; 1],
            last_byte: None,
            no_more_bytes: false,
        }
    }
}

impl<'a> Iterator for NibbleIter<'a> {
    type Item = Nibble;

    fn next(&mut self) -> Option<Nibble> {
        let nibble = {
            match self.last_byte {
                None => {
                    if self.no_more_bytes {
                        return None;
                    }

                    if self.cursor.position() >= self.length {
                        self.no_more_bytes = true;
                        return None;
                    }

                    if let Err(..) = self.cursor.read_exact(&mut self.buf) {
                        panic!("Not enough bytes")
                    }

                    let byte = self.buf[0];
                    self.last_byte = Some(byte);

                    Nibble((byte & 0xF0) >> 4)
                }
                Some(byte) => {
                    self.last_byte = None;
                    Nibble(byte & 0x0F)
                }
            }
        };

        Some(nibble)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_nibble(iter: &mut NibbleIter) -> u8 {
        iter.next().unwrap().0
    }

    fn maybe_read_nibble(iter: &mut NibbleIter) -> Option<u8> {
        iter.next().map(|nibble| nibble.0)
    }

    #[test]
    fn nibble_iter_reads_empty_seq() {
        let vec = vec![];
        let mut iter = NibbleIter::new(&vec[..]);

        assert_eq!(None, maybe_read_nibble(&mut iter));
    }

    #[test]
    fn nibble_iter_reads_nibbles() {
        let vec = vec![0b_1001_1111, 0b_0011_0000];
        let mut iter = NibbleIter::new(&vec[..]);

        assert_eq!(0b_0000_1001, read_nibble(&mut iter));
        assert_eq!(0b_0000_1111, read_nibble(&mut iter));
        assert_eq!(0b_0000_0011, read_nibble(&mut iter));
        assert_eq!(0b_0000_0000, read_nibble(&mut iter));
        assert_eq!(None, maybe_read_nibble(&mut iter));
    }
}
