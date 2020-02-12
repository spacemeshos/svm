use std::{
    io::{Cursor, Read},
    iter::Iterator,
};

use super::nibble::{concat_nibbles, Nibble};

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

                    // given `byte` is `lnibble | rnibble`
                    // we return the left nibble encoded as a byte in the form:
                    // `0b_0000_{lnibble}`

                    Nibble::new((byte & 0xF0) >> 4)
                }
                Some(byte) => {
                    self.last_byte = None;
                    Nibble::new(byte & 0x0F)
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
        iter.next().unwrap().inner()
    }

    fn maybe_read_nibble(iter: &mut NibbleIter) -> Option<u8> {
        iter.next().map(|nibble| nibble.inner())
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
