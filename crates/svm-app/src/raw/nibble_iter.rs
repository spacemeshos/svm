use std::{
    io::{Cursor, Read},
    iter::Iterator,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Nibble(pub u8);

pub struct NibbleIter<'a, 'b: 'a> {
    buf: [u8; 1],
    cursor: &'a mut Cursor<&'b [u8]>,
    current_byte: Option<u8>,
}

impl<'a, 'b> NibbleIter<'a, 'b> {
    pub fn new(cursor: &'a mut Cursor<&'b [u8]>) -> Self {
        Self {
            cursor,
            current_byte: None,
            buf: [0; 1],
        }
    }
}

impl<'a, 'b> Iterator for NibbleIter<'a, 'b> {
    type Item = Nibble;

    fn next(&mut self) -> Option<Nibble> {
        match self.current_byte {
            None => {
                self.cursor
                    .read_exact(&mut self.buf)
                    .expect("Not bytes to read");

                let byte = self.buf[0];
                self.current_byte = Some(byte);

                let nibble = Nibble((byte & 0xF0) >> 4);
                Some(nibble)
            }
            Some(byte) => {
                self.current_byte = None;

                let nibble = Nibble(byte & 0x0F);
                Some(nibble)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_nibble(iter: &mut NibbleIter) -> u8 {
        iter.next().unwrap().0
    }

    #[test]
    fn nibble_iter_reads_nibbles() {
        let vec = vec![0b_1001_1111, 0b_0011_0000];
        let mut cursor = Cursor::new(&vec[..]);

        let mut iter = NibbleIter::new(&mut cursor);

        assert_eq!(0b_0000_1001, read_nibble(&mut iter));
        assert_eq!(0b_0000_1111, read_nibble(&mut iter));
        assert_eq!(0b_0000_0011, read_nibble(&mut iter));
        assert_eq!(0b_0000_0000, read_nibble(&mut iter));
    }
}
