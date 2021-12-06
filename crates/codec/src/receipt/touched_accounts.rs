use std::collections::HashSet;

use svm_types::{Address, BytesPrimitive};

use crate::{Codec, ParseError, ReadExt, WriteExt};

impl Codec for HashSet<Address> {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        u16::try_from(self.len()).unwrap().encode(w);

        for addr in self.iter() {
            w.write_bytes(addr.as_slice());
        }
    }

    fn decode(cursor: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let len = u16::decode(cursor)?;
        let mut touched_accounts = HashSet::new();

        for _ in 0..len {
            touched_accounts.insert(Address::new(cursor.read_bytes(20)?));
        }

        Ok(touched_accounts)
    }
}
