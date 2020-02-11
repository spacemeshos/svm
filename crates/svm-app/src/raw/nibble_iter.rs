use std::{
    io::{Cursor, Read},
    iter::Iterator,
};

use super::nibble::Nibble;

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

impl<'a> NibbleIter<'a> {
    pub fn read_bytes(&mut self, byte_count: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(byte_count);

        for _ in 0..byte_count {
            let lnib = self.next();
            let rnib = self.next();

            match (lnib, rnib) {
                (Some(lnib), Some(rnib)) => {
                    let byte = (lnib.0 << 4) | rnib.0;
                    bytes.push(byte);
                }
                (Some(lnib), None) => {
                    bytes.push(lnib.0);
                }
                (None, None) => {
                    //
                }
                _ => unreachable!(),
            }
        }

        bytes
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

    #[test]
    fn nibble_iter_read_byte_a_time() {
        let vec = vec![0b_1001_1111, 0b_0011_0111];
        let mut iter = NibbleIter::new(&vec[..]);

        assert_eq!(vec![0b_1001_1111], iter.read_bytes(1));
        assert_eq!(vec![0b_0011_0111], iter.read_bytes(1));
        assert!(iter.read_bytes(1).is_empty());
    }

    #[test]
    fn nibble_iter_read_two_bytes_a_time() {
        let vec = vec![0b_1001_1111, 0b_0011_0111, 0b_1100_0110];
        let mut iter = NibbleIter::new(&vec[..]);

        assert_eq!(vec![0b_1001_1111, 0b_0011_0111], iter.read_bytes(2));
        assert_eq!(vec![0b_1100_0110], iter.read_bytes(2));
        assert!(iter.read_bytes(2).is_empty());
    }
}
