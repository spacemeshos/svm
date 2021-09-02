use std::convert::TryInto;
use std::ops::Range;

use svm_codec::Codec;
use svm_codec::{ParseError, ReadExt, WriteExt};
use svm_layout::{FixedLayout, Id};
use svm_types::{Address, BytesPrimitive, TemplateAddr};

use crate::TemplateStorage;
use crate::{GlobalState, StorageResult};

pub struct AccountStorage {
    gs: GlobalState,
    address: Address,
    template_addr: TemplateAddr,
    layout: FixedLayout,
}

impl AccountStorage {
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

    pub fn create(
        &mut self,
        name: String,
        template_addr: TemplateAddr,
        balance: u64,
        counter: u64,
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

    pub fn gs(&self) -> &GlobalState {
        &self.gs
    }

    pub fn gs_mut(&mut self) -> &mut GlobalState {
        &mut self.gs
    }

    /// # Panics
    ///
    /// Panics if `var` is empty.
    pub fn get_var(&self, var_id: u32, mut var: &mut [u8]) -> StorageResult<()> {
        let raw_var = self.layout.get(Id(var_id));
        let offset = raw_var.offset();
        let byte_size = raw_var.byte_size();

        assert_eq!(var.len(), byte_size as usize);

        let segments = var_segments(&self.address, offset, byte_size);

        for segment in segments.into_iter() {
            let bytes: [u8; 32] = self
                .gs
                .block_on(self.gs.storage().get(segment.key.as_bytes(), None))?
                .unwrap_or(vec![0; 32])
                .try_into()
                .expect("Unexpected length of value, expected 32 bytes.");

            var[..segment.range.len()].copy_from_slice(&bytes[segment.range.clone()]);
            var = &mut var[segment.range.len()..];
        }

        Ok(())
    }

    pub fn get_var_vec(&self, var_id: u32) -> StorageResult<Vec<u8>> {
        let raw_var = self.layout.get(Id(var_id));
        let mut bytes = vec![0; raw_var.byte_size() as usize];
        self.get_var(var_id, &mut bytes)?;

        Ok(bytes)
    }

    pub fn get_var_i64(&self, var_id: u32) -> i64 {
        let mut bytes = [0; 8];
        self.get_var(var_id, &mut bytes);

        i64::from_be_bytes(bytes)
    }

    pub fn get_var_i32(&self, var_id: u32) -> i32 {
        let mut bytes = [0; 4];
        self.get_var(var_id, &mut bytes);

        i32::from_be_bytes(bytes)
    }

    pub fn get_var_160(&self, var_id: u32) -> [u8; 20] {
        let mut bytes = [0; 20];
        self.get_var(var_id, &mut bytes);

        bytes
    }

    pub fn set_var(&mut self, var_id: u32, mut new_value: &[u8]) -> StorageResult<()> {
        let raw_var = self.layout.get(Id(var_id));
        let offset = raw_var.offset();
        let byte_size = raw_var.byte_size();

        let segments = var_segments(&self.address, offset, byte_size);

        for segment in segments.into_iter() {
            let mut bytes: [u8; 32] = self
                .gs
                .block_on(self.gs.storage().get(segment.key.as_bytes(), None))?
                .unwrap_or(vec![0; 32])
                .try_into()
                .expect("Unexpected length of value, expected 32 bytes.");

            bytes[segment.range.clone()].copy_from_slice(&new_value[..segment.range.len()]);
            new_value = &new_value[segment.range.len()..];

            self.gs.storage().upsert(segment.key.as_bytes(), &bytes[..]);
        }

        Ok(())
    }

    pub fn set_var_160(&mut self, var_id: u32, new_value: [u8; 20]) -> StorageResult<()> {
        self.set_var(var_id, &new_value[..])
    }

    pub fn set_var_i64(&mut self, var_id: u32, new_value: i64) -> StorageResult<()> {
        self.set_var(var_id, &new_value.to_be_bytes()[..])
    }

    pub fn template_storage(&self) -> TemplateStorage {
        TemplateStorage::new(&self.template_addr, self.gs.clone())
    }

    pub fn name(&self, account_addr: &Address) -> StorageResult<Option<String>> {
        self.gs
            .read_and_decode::<AccountData>(&AccountData::key(account_addr))
            .map(|res| res.map(|data| data.name))
    }

    pub fn template_addr(&self, account_addr: &Address) -> StorageResult<Option<TemplateAddr>> {
        self.gs
            .read_and_decode::<AccountData>(&AccountData::key(account_addr))
            .map(|res| res.map(|data| data.template_addr))
    }

    /// Reads and returns the balance of `account_addr`.
    pub fn balance(&self, account_addr: &Address) -> StorageResult<Option<u64>> {
        self.gs
            .read_and_decode::<AccountMut>(&AccountMut::key(account_addr))
            .map(|res| res.map(|data| data.balance))
    }

    /// Reads and returns the nonce counter of `account_addr`.
    pub fn counter(&self, account_addr: &Address) -> StorageResult<Option<u64>> {
        self.gs
            .read_and_decode::<AccountMut>(&AccountMut::key(account_addr))
            .map(|res| res.map(|data| data.counter))
    }

    pub fn set_balance(&mut self, account_addr: &Address, balance: u64) -> StorageResult<()> {
        self.gs
            .replace(&AccountMut::key(account_addr), |mut data: AccountMut| {
                data.balance = balance;
                data
            })
    }

    pub fn set_counter(&mut self, account_addr: &Address, counter: u64) -> StorageResult<()> {
        self.gs
            .replace(&AccountMut::key(account_addr), |mut data: AccountMut| {
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

struct Segment {
    key: String,
    range: Range<usize>,
}

fn var_segments(account_addr: &Address, offset: u32, byte_size: u32) -> Vec<Segment> {
    let mut segment_index = offset / 32;
    let mut segments = vec![];

    let first_range = (offset % 32) as usize..((offset % 32) + byte_size).min(32) as usize;
    segments.push(Segment {
        key: key_account_var_segment(account_addr, segment_index),
        range: first_range,
    });

    let mut byte_size = 32i64 - (offset % 32) as i64;

    segment_index += 1;

    while byte_size > 0 {
        segments.push(Segment {
            key: key_account_var_segment(account_addr, segment_index),
            range: 0..32.min(byte_size as usize),
        });

        segment_index += 1;
        byte_size -= 32;
    }

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
        let encoding_version = 0u8;

        encoding_version.encode(w);
        self.template_addr.encode(w);
        self.name.encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let encoding_version = u8::decode(reader)?;

        if encoding_version != 0 {
            return Err(ParseError::BadByte(encoding_version));
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
    pub counter: u64,
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
        let counter = u64::decode(reader)?;

        Ok(Self { balance, counter })
    }
}
