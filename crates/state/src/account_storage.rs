use std::convert::TryInto;
use std::ops::RangeInclusive;

use svm_codec::Codec;
use svm_codec::{ParseError, ReadExt, WriteExt};
use svm_layout::{FixedLayout, Id};
use svm_types::{Address, BytesPrimitive, TemplateAddr};

use crate::TemplateStorage;
use crate::{GlobalState, StorageResult};

const SEGMENT_SIZE: usize = 32;

/// A [`GlobalState`] wrapper, enriched with utility methods to access and
/// modify [`Account`](svm_types::Account) data.
#[derive(Debug, Clone)]
pub struct AccountStorage {
    /// The internal [`GlobalState`] instance used to access the database layer.
    pub gs: GlobalState,

    address: Address,
    template_addr: TemplateAddr,
    layout: FixedLayout,
}

impl AccountStorage {
    /// Creates a new [`AccountStorage`].
    pub fn new(
        gs: GlobalState,
        address: &Address,
        template_addr: &TemplateAddr,
        layout: &FixedLayout,
    ) -> Self {
        Self {
            gs,
            address: address.clone(),
            template_addr: template_addr.clone(),
            layout: layout.clone(),
        }
    }

    /// Saves `self` to the associated [`GlobalState`].
    pub fn create(
        &mut self,
        name: String,
        template_addr: TemplateAddr,
        balance: u64,
        counter: u128,
    ) {
        self.gs.encode_and_write(
            &AccountData {
                name,
                template_addr,
            },
            &AccountData::key(&self.address),
        );

        self.gs.encode_and_write(
            &AccountMut { balance, counter },
            &AccountMut::key(&self.address),
        );
    }

    /// Reads `var_id` from the storage layer and writes its contents into
    /// `var`.
    ///
    /// In case `var` is larger than necessary, only the first relevant bytes
    /// are overwritten.
    ///
    /// # Panics
    ///
    /// Panics if `var` is not large enough to hold the `var_id` value.
    pub fn get_var(&self, var_id: u32, mut var: &mut [u8]) -> StorageResult<()> {
        let raw_var = self.layout.get(Id(var_id));
        let offset = raw_var.offset();
        let byte_size = raw_var.byte_size();

        assert!(var.len() >= byte_size as usize);
        var = &mut var[..byte_size as usize];

        let segments = var_segments(
            &self.address,
            offset + byte_size - var.len() as u32,
            var.len() as u32,
        );

        for segment in segments.into_iter() {
            let bytes: [u8; SEGMENT_SIZE] = self
                .gs
                .block_on(self.gs.storage().get(segment.key.as_bytes(), None))?
                .unwrap_or(vec![0; SEGMENT_SIZE])
                .try_into()
                .expect("Unexpected length of value.");

            var[..segment.len()].copy_from_slice(&bytes[segment.range.clone()]);
            var = &mut var[segment.len()..];
        }

        Ok(())
    }

    /// Reads and returns the data associated with `var_id` in a [`Vec<u8>`].
    pub fn get_var_vec(&self, var_id: u32) -> StorageResult<Vec<u8>> {
        let raw_var = self.layout.get(Id(var_id));
        let mut bytes = vec![0; raw_var.byte_size() as usize];

        self.get_var(var_id, &mut bytes)?;

        Ok(bytes)
    }

    /// Reads and returns the an array of bytes that holds the value associated
    /// with `var_id`.
    ///
    /// In case `var` is larger
    /// than necessary, only the leading relevant bytes are overwritten.
    ///
    /// # Panics
    ///
    /// Panics if `N` is not large enough to hold the `var_id` value.
    pub fn get_var_array<const N: usize>(&self, var_id: u32) -> StorageResult<[u8; N]> {
        let mut bytes = [0; N];
        self.get_var(var_id, &mut bytes)?;

        Ok(bytes)
    }

    /// Reads and returns the [`i64`] value associated with `var_id`.
    pub fn get_var_i64(&self, var_id: u32) -> StorageResult<i64> {
        let mut bytes = [0; 8];
        self.get_var(var_id, &mut bytes)?;

        Ok(i64::from_le_bytes(bytes))
    }

    /// Reads and returns the [`i32`] value associated with `var_id`.
    pub fn get_var_i32(&self, var_id: u32) -> StorageResult<i32> {
        let mut bytes = [0; 4];
        self.get_var(var_id, &mut bytes)?;

        Ok(i32::from_le_bytes(bytes))
    }

    /// Replaces the `var_id` value in the storage layer with the contents of
    /// `new_value`.
    ///
    /// In case `new_value` is larger than necessary, only the first relevant
    /// bytes are considered.
    ///
    /// # Panics
    ///
    /// Panics if `new_value` is not large enough to contain a fully-qualified
    /// `var_id` value.
    pub fn set_var_bytes(&mut self, var_id: u32, mut new_value: &[u8]) -> StorageResult<()> {
        let raw_var = self.layout.get(Id(var_id));
        let offset = raw_var.offset();
        let byte_size = raw_var.byte_size();

        assert!(new_value.len() >= byte_size as usize);
        new_value = &new_value[..byte_size as usize];

        let segments = var_segments(
            &self.address,
            offset + byte_size - new_value.len() as u32,
            new_value.len() as u32,
        );

        for segment in segments.into_iter() {
            let mut bytes: [u8; SEGMENT_SIZE] = self
                .gs
                .block_on(self.gs.storage().get(segment.key.as_bytes(), None))?
                .unwrap_or(vec![0; SEGMENT_SIZE])
                .try_into()
                .expect("Unexpected length of value.");

            bytes[segment.range.clone()].copy_from_slice(&new_value[..segment.len()]);
            new_value = &new_value[segment.len()..];

            self.gs
                .block_on(self.gs.storage().upsert(segment.key.as_bytes(), &bytes[..]));
        }

        Ok(())
    }

    /// Overwrites the [`i64`] value associated with `var_id`.
    pub fn set_var_i64(&mut self, var_id: u32, new_value: i64) -> StorageResult<()> {
        self.set_var_bytes(var_id, &new_value.to_le_bytes()[..])
    }

    /// Overwrites the [`i32`] value associated with `var_id`.
    pub fn set_var_i32(&mut self, var_id: u32, new_value: i32) -> StorageResult<()> {
        self.set_var_bytes(var_id, &new_value.to_le_bytes()[..])
    }

    /// Creates a new [`TemplateStorage`] utility instance for the
    /// [`Template`](svm_types::Template) of this
    /// [`Account`](svm_types::Account).
    pub fn template_storage(&self) -> TemplateStorage {
        TemplateStorage::new(&self.template_addr, self.gs.clone())
    }

    /// Reads and returns the [`Account`](svm_types::Account) name of
    /// `self`.
    pub fn name(&self) -> StorageResult<Option<String>> {
        self.gs
            .read_and_decode::<AccountData>(&AccountData::key(&self.address))
            .map(|res| res.map(|data| data.name))
    }

    /// Reads and returns the [`TemplateAddr`] of `self`.
    pub fn template_addr(&self) -> StorageResult<Option<TemplateAddr>> {
        self.gs
            .read_and_decode::<AccountData>(&AccountData::key(&self.address))
            .map(|res| res.map(|data| data.template_addr))
    }

    /// Reads and returns the balance of `self`.
    pub fn balance(&self) -> StorageResult<Option<u64>> {
        self.gs
            .read_and_decode::<AccountMut>(&AccountMut::key(&self.address))
            .map(|res| res.map(|data| data.balance))
    }

    /// Reads and returns the nonce counter of `self`.
    pub fn counter(&self) -> StorageResult<Option<u128>> {
        self.gs
            .read_and_decode::<AccountMut>(&AccountMut::key(&self.address))
            .map(|res| res.map(|data| data.counter))
    }

    /// Replaces the current balance of `self`.
    pub fn set_balance(&mut self, balance: u64) -> StorageResult<()> {
        self.gs
            .replace(&AccountMut::key(&self.address), |mut data: AccountMut| {
                data.balance = balance;
                data
            })
    }

    /// Replaces the current nonce counter of `self`.
    pub fn set_counter(&mut self, counter: u128) -> StorageResult<()> {
        self.gs
            .replace(&AccountMut::key(&self.address), |mut data: AccountMut| {
                data.counter = counter;
                data
            })
    }
}

fn key_account_var_segment(account_addr: &Address, segment: u32) -> String {
    format!(
        "accounts:{}:var_segments:{}",
        account_addr.to_string(),
        segment
    )
}

#[derive(Debug)]
struct Segment {
    key: String,
    range: RangeInclusive<usize>,
}

impl Segment {
    fn len(&self) -> usize {
        // E.g. `0..=2` contains 3 elements.
        *self.range.end() - *self.range.start() + 1
    }
}

fn var_segments(account_addr: &Address, offset: u32, byte_size: u32) -> Vec<Segment> {
    let mut remaining_size = i64::from(byte_size);
    let mut segments = vec![];
    let mut segment_index = offset as usize / SEGMENT_SIZE;
    let mut segment_start = offset as usize % SEGMENT_SIZE;
    let mut segment_end = (segment_start + byte_size as usize - 1).min(SEGMENT_SIZE - 1);

    segments.push(Segment {
        key: key_account_var_segment(account_addr, segment_index as u32),
        range: segment_start..=segment_end,
    });

    remaining_size -= segment_end as i64 - segment_start as i64 + 1;
    segment_start = 0;

    while remaining_size > 0 {
        segment_index += 1;
        segment_end = (SEGMENT_SIZE - 1).min(remaining_size as usize - 1);
        remaining_size -= SEGMENT_SIZE as i64;

        segments.push(Segment {
            key: key_account_var_segment(account_addr, segment_index as u32),
            range: segment_start..=segment_end,
        });
    }

    debug_assert_eq!(
        segments.iter().map(|s| s.len()).sum::<usize>(),
        byte_size as usize
    );

    segments
}

struct AccountData {
    pub name: String,
    pub template_addr: TemplateAddr,
}

impl AccountData {
    pub fn key(account_addr: &Address) -> String {
        format!("accounts:{}:immutable", account_addr.to_string())
    }
}

impl Codec for AccountData {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        let version = 0u8;
        version.encode(w);

        self.template_addr.encode(w);
        self.name.encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let version = u8::decode(reader)?;

        if version != 0 {
            return Err(ParseError::BadByte(version));
        }

        let template_addr = TemplateAddr::decode(reader)?;
        let name = String::decode(reader)?;

        Ok(Self {
            name,
            template_addr,
        })
    }
}

pub struct AccountMut {
    pub balance: u64,
    pub counter: u128,
}

impl AccountMut {
    pub fn key(account_addr: &Address) -> String {
        format!("accounts:{}:mutable", account_addr.to_string())
    }
}

impl Codec for AccountMut {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        let encoding_version = 0u8;

        encoding_version.encode(w);
        self.balance.encode(w);
        self.counter.encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let encoding_version = u8::decode(reader)?;

        if encoding_version != 0 {
            return Err(ParseError::BadByte(encoding_version));
        }

        let balance = u64::decode(reader)?;
        let counter = u128::decode(reader)?;

        Ok(Self { balance, counter })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn fixed_layout() -> FixedLayout {
        FixedLayout::from(vec![10, 20, 4, 30, 64, 31, 100, 4, 8, 8])
    }

    #[test]
    fn immutable_metadata() {
        let layout = fixed_layout();
        let address = Address::repeat(0xff);
        let template_addr = TemplateAddr::repeat(0x80);
        let name = "@name";
        let balance = 42;
        let counter = 0;

        let gs = GlobalState::in_memory();
        let mut account = AccountStorage::new(gs, &address, &template_addr, &layout);
        account.create(name.to_string(), template_addr, balance, counter);

        assert_eq!(account.name().unwrap().unwrap(), name);
        assert_eq!(account.template_addr().unwrap().unwrap(), template_addr);
        assert_eq!(account.balance().unwrap().unwrap(), balance);
        assert_eq!(account.counter().unwrap().unwrap(), counter);
    }

    #[test]
    fn mutable_metadata() {
        let layout = fixed_layout();
        let address = Address::repeat(0xff);
        let template_addr = TemplateAddr::repeat(0x80);
        let name = "@name";
        let balance = 42;
        let counter = 0;

        let gs = GlobalState::in_memory();
        let mut account = AccountStorage::new(gs, &address, &template_addr, &layout);
        account.create(name.to_string(), template_addr, balance, counter);

        assert_eq!(account.balance().unwrap().unwrap(), balance);
        assert_eq!(account.counter().unwrap().unwrap(), counter);

        account.set_balance(1000).unwrap();

        assert_eq!(account.balance().unwrap().unwrap(), 1000);
        assert_eq!(account.counter().unwrap().unwrap(), counter);

        account.set_counter(10).unwrap();
        assert_eq!(account.balance().unwrap().unwrap(), 1000);
        assert_eq!(account.counter().unwrap().unwrap(), 10);

        account.set_counter(100).unwrap();
        assert_eq!(account.balance().unwrap().unwrap(), 1000);
        assert_eq!(account.counter().unwrap().unwrap(), 100);
    }

    #[test]
    fn account_byte_vars() {
        let layout = fixed_layout();
        let address = Address::repeat(0xff);
        let template_addr = TemplateAddr::repeat(0x80);
        let name = "@name";
        let balance = 42;
        let counter = 0;

        let gs = GlobalState::in_memory();
        let mut account = AccountStorage::new(gs, &address, &template_addr, &layout);
        account.create(name.to_string(), template_addr, balance, counter);

        account.set_var_bytes(1, &[1; 10]).unwrap();
        account.set_var_bytes(2, &[2; 20]).unwrap();
        account.set_var_bytes(3, &[3; 4]).unwrap();
        account.set_var_bytes(4, &[4; 30]).unwrap();
        account.set_var_bytes(5, &[5; 64]).unwrap();
        account.set_var_bytes(6, &[6; 31]).unwrap();
        account.set_var_bytes(7, &[7; 100]).unwrap();

        assert_eq!(account.get_var_vec(7).unwrap(), &[7; 100]);
        assert_eq!(account.get_var_vec(6).unwrap(), &[6; 31]);
        assert_eq!(account.get_var_vec(5).unwrap(), &[5; 64]);
        assert_eq!(account.get_var_vec(4).unwrap(), &[4; 30]);
        assert_eq!(account.get_var_vec(3).unwrap(), &[3; 4]);
        assert_eq!(account.get_var_vec(2).unwrap(), &[2; 20]);
        assert_eq!(account.get_var_vec(1).unwrap(), &[1; 10]);
    }

    #[test]
    fn account_numeric_vars() {
        let layout = fixed_layout();
        let address = Address::repeat(0xff);
        let template_addr = TemplateAddr::repeat(0x80);
        let name = "@name";
        let balance = 42;
        let counter = 0;

        let gs = GlobalState::in_memory();
        let mut account = AccountStorage::new(gs, &address, &template_addr, &layout);
        account.create(name.to_string(), template_addr, balance, counter);

        account.set_var_i32(8, -20414).unwrap();
        account.set_var_i64(9, 1337).unwrap();
        account.set_var_i64(10, i64::MAX).unwrap();

        assert_eq!(account.get_var_i32(8).unwrap(), -20414);
        assert_eq!(account.get_var_i64(9).unwrap(), 1337);
        assert_eq!(account.get_var_i64(10).unwrap(), i64::MAX);
    }
}
