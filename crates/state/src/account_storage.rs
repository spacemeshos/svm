use std::convert::TryInto;
use std::ops::RangeInclusive;

use svm_codec::{Codec, ParseError, ReadExt, WriteExt};
use svm_layout::FixedLayout;
use svm_types::{Address, BytesPrimitive, SectionKind, TemplateAddr};

use crate::TemplateStorage;
use crate::{GlobalState, StorageResult};

const SEGMENT_SIZE: usize = 32;

/// A [`GlobalState`] wrapper, enriched with utility methods to access and
/// modify [`Account`](svm_types::Account) data.
#[derive(Debug, Clone)]
pub struct AccountStorage {
    /// The internal [`GlobalState`] instance used to access the database layer.
    pub gs: GlobalState,

    /// The owner's [`Address`] of this [`AccountStorage`].
    pub address: Address,
    template_addr: TemplateAddr,
    layout: FixedLayout,
}

impl AccountStorage {
    /// Saves `self` to the associated [`GlobalState`].
    pub fn create(
        mut gs: GlobalState,
        address: &Address,
        name: String,
        template_addr: TemplateAddr,
        balance: u64,
        counter: u128,
    ) -> StorageResult<Self> {
        let layout = template_layout(gs.clone(), &template_addr)?;

        gs.encode_and_write(
            &AccountData {
                name,
                template_addr,
            },
            &AccountData::key(address),
        );

        gs.encode_and_write(&AccountMut { balance, counter }, &AccountMut::key(address));

        Ok(Self {
            gs,
            address: address.clone(),
            template_addr,
            layout,
        })
    }

    /// Creates a new [`AccountStorage`].
    pub fn load(gs: GlobalState, address: &Address) -> StorageResult<Self> {
        let account_data = AccountData::read(&gs, address)?;
        let layout = template_layout(gs.clone(), &account_data.template_addr)?;

        Ok(Self {
            gs,
            address: address.clone(),
            template_addr: account_data.template_addr,
            layout,
        })
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
        let raw_var = self.layout.get(var_id);
        let offset = raw_var.offset;
        let byte_size = raw_var.byte_size;

        if var.len() < byte_size as usize {
            panic!("The given buffer is not large enough");
        } else if var.len() > byte_size as usize {
            var = &mut var[..byte_size as usize];
        }

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
        let raw_var = self.layout.get(var_id);
        let mut bytes = vec![0; raw_var.byte_size as usize];

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
        let raw_var = self.layout.get(var_id);
        let offset = raw_var.offset;
        let byte_size = raw_var.byte_size;

        if new_value.len() < byte_size as usize {
            panic!("The given buffer is not large enough");
        } else if new_value.len() > byte_size as usize {
            new_value = &new_value[..byte_size as usize];
        }

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
    ///
    /// # Panics
    ///
    /// Panics if `self` was originated from a God template.
    pub fn template_storage(&self) -> StorageResult<TemplateStorage> {
        if self.template_addr == TemplateAddr::god_template() {
            panic!("Can't get template data associated with a God template!");
        }
        TemplateStorage::load(self.gs.clone(), &self.template_addr)
    }

    /// Reads and returns the [`Account`](svm_types::Account) name of
    /// `self`.
    pub fn name(&self) -> StorageResult<String> {
        let key = AccountData::key(&self.address);

        Ok(self.gs.read_and_decode::<AccountData>(key.as_str())?.name)
    }

    /// Reads and returns the [`TemplateAddr`] of `self`.
    pub fn template_addr(&self) -> StorageResult<TemplateAddr> {
        self.gs
            .read_and_decode::<AccountData>(&AccountData::key(&self.address))
            .map(|data| data.template_addr)
    }

    /// Reads and returns the balance of `self`.
    pub fn balance(&self) -> StorageResult<u64> {
        self.gs
            .read_and_decode::<AccountMut>(&AccountMut::key(&self.address))
            .map(|data| data.balance)
    }

    /// Reads and returns the nonce counter of `self`.
    pub fn counter(&self) -> StorageResult<u128> {
        self.gs
            .read_and_decode::<AccountMut>(&AccountMut::key(&self.address))
            .map(|data| data.counter)
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

fn template_layout(gs: GlobalState, template_addr: &TemplateAddr) -> StorageResult<FixedLayout> {
    if *template_addr == TemplateAddr::god_template() {
        Ok(FixedLayout::default())
    } else {
        let template_storage = TemplateStorage::load(gs, &template_addr)?;
        let sections = template_storage.sections()?;
        let data_section = sections.get(SectionKind::Data).as_data();
        let layout = data_section.layouts()[0].as_fixed().clone();
        Ok(layout)
    }
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

    pub fn read(gs: &GlobalState, address: &Address) -> StorageResult<Self> {
        gs.read_and_decode::<Self>(&Self::key(address))
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
    use svm_layout::Layout;
    use svm_types::{CodeSection, CtorsSection, DataSection, Sections, Template};

    use super::*;

    fn fixed_layout() -> FixedLayout {
        FixedLayout::from_byte_sizes(0, &[10, 20, 4, 30, 64, 31, 100, 4, 8, 8])
    }

    fn new_template(gs: &GlobalState) -> TemplateAddr {
        let template_addr = TemplateAddr::repeat(0x80);

        let code_section = CodeSection::new(
            svm_types::CodeKind::Wasm,
            vec![],
            0,
            svm_types::GasMode::Fixed,
            0,
        );
        let data_section = DataSection::with_layout(Layout::Fixed(fixed_layout()));
        let ctors_section = CtorsSection::new(vec![]);

        let core_sections = Template::new(code_section, data_section, ctors_section)
            .sections()
            .clone();
        let noncore_sections = Sections::with_capacity(0);

        TemplateStorage::create(gs.clone(), &template_addr, core_sections, noncore_sections)
            .unwrap();

        template_addr
    }

    #[test]
    fn immutable_metadata() {
        let gs = GlobalState::in_memory();

        let address = Address::repeat(0xff);
        let template_addr = new_template(&gs);
        let name = "@name";
        let balance = 42;
        let counter = 0;

        let account = AccountStorage::create(
            gs,
            &address,
            name.to_string(),
            template_addr,
            balance,
            counter,
        )
        .unwrap();

        assert_eq!(account.name().unwrap(), name);
        assert_eq!(account.template_addr().unwrap(), template_addr);
        assert_eq!(account.balance().unwrap(), balance);
        assert_eq!(account.counter().unwrap(), counter);
    }

    #[test]
    fn mutable_metadata() {
        let gs = GlobalState::in_memory();

        let address = Address::repeat(0xff);
        let template_addr = new_template(&gs);
        let name = "@name";
        let balance = 42;
        let counter = 0;

        let mut account = AccountStorage::create(
            gs,
            &address,
            name.to_string(),
            template_addr,
            balance,
            counter,
        )
        .unwrap();

        assert_eq!(account.balance().unwrap(), balance);
        assert_eq!(account.counter().unwrap(), counter);

        account.set_balance(1000).unwrap();

        assert_eq!(account.balance().unwrap(), 1000);
        assert_eq!(account.counter().unwrap(), counter);

        account.set_counter(10).unwrap();
        assert_eq!(account.balance().unwrap(), 1000);
        assert_eq!(account.counter().unwrap(), 10);

        account.set_counter(100).unwrap();
        assert_eq!(account.balance().unwrap(), 1000);
        assert_eq!(account.counter().unwrap(), 100);
    }

    #[test]
    fn account_byte_vars() {
        let gs = GlobalState::in_memory();

        let address = Address::repeat(0xff);
        let template_addr = new_template(&gs);
        let name = "@name";
        let balance = 42;
        let counter = 0;

        let mut account = AccountStorage::create(
            gs,
            &address,
            name.to_string(),
            template_addr,
            balance,
            counter,
        )
        .unwrap();

        account.set_var_bytes(0, &[1; 10]).unwrap();
        account.set_var_bytes(1, &[2; 20]).unwrap();
        account.set_var_bytes(2, &[3; 4]).unwrap();
        account.set_var_bytes(3, &[4; 30]).unwrap();
        account.set_var_bytes(4, &[5; 64]).unwrap();
        account.set_var_bytes(5, &[6; 31]).unwrap();
        account.set_var_bytes(6, &[7; 100]).unwrap();

        assert_eq!(account.get_var_vec(6).unwrap(), &[7; 100]);
        assert_eq!(account.get_var_vec(5).unwrap(), &[6; 31]);
        assert_eq!(account.get_var_vec(4).unwrap(), &[5; 64]);
        assert_eq!(account.get_var_vec(3).unwrap(), &[4; 30]);
        assert_eq!(account.get_var_vec(2).unwrap(), &[3; 4]);
        assert_eq!(account.get_var_vec(1).unwrap(), &[2; 20]);
        assert_eq!(account.get_var_vec(0).unwrap(), &[1; 10]);
    }

    #[test]
    fn account_numeric_vars() {
        let gs = GlobalState::in_memory();

        let address = Address::repeat(0xff);
        let template_addr = new_template(&gs);
        let name = "@name";
        let balance = 42;
        let counter = 0;

        let mut account = AccountStorage::create(
            gs,
            &address,
            name.to_string(),
            template_addr,
            balance,
            counter,
        )
        .unwrap();

        account.set_var_i32(7, -20414).unwrap();
        account.set_var_i64(8, 1337).unwrap();
        account.set_var_i64(9, i64::MAX).unwrap();

        assert_eq!(account.get_var_i32(7).unwrap(), -20414);
        assert_eq!(account.get_var_i64(8).unwrap(), 1337);
        assert_eq!(account.get_var_i64(9).unwrap(), i64::MAX);
    }

    #[test]
    fn create_then_load() {
        let gs = GlobalState::in_memory();

        let address = Address::repeat(0xff);
        let template_addr = new_template(&gs);
        let name = "@name";
        let balance = 42;
        let counter = 0;

        let account = AccountStorage::create(
            gs.clone(),
            &address,
            name.to_string(),
            template_addr,
            balance,
            counter,
        )
        .unwrap();

        let new_account = AccountStorage::load(gs, &address).unwrap();

        assert_eq!(account.name().unwrap(), new_account.name().unwrap());
        assert_eq!(
            account.template_addr().unwrap(),
            new_account.template_addr().unwrap()
        );
        assert_eq!(account.balance().unwrap(), new_account.balance().unwrap());
    }
}
