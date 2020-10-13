mod ext;
mod mock;
mod traits;

pub use ext::ExtStorage;
pub use mock::MockStorage;
pub use traits::Storage;

use crate::Amount;

macro_rules! use_storage {
    () => {
        #[cfg(test)]
        use MockStorage as Storage;

        #[cfg(not(test))]
        use ExtStorage as Storage;
    };
}

pub fn get32(var_id: u32) -> u32 {
    use_storage!();

    Storage::get32(var_id)
}

pub fn set32(var_id: u32, value: u32) {
    use_storage!();

    Storage::set32(var_id, value)
}

pub fn get64(var_id: u32) -> u64 {
    use_storage!();

    Storage::get64(var_id)
}

pub fn set64(var_id: u32, value: u64) {
    use_storage!();

    Storage::set64(var_id, value)
}

pub fn get_bool(var_id: u32) -> bool {
    use_storage!();

    let value = Storage::get32(var_id);
    int_to_bool(value)
}

pub fn set_bool(var_id: u32, value: bool) {
    use_storage!();

    let value = bool_to_int(value);
    Storage::set32(var_id, value)
}

pub fn get_amount(var_id: u32) -> Amount {
    let value = get64(var_id);

    Amount(value)
}

pub fn set_amount(var_id: u32, value: Amount) {
    let value = value.0;

    set64(var_id, value);
}

// Array get_bool / set_bool

pub fn array_get_bool(var_id: u32, index: u32, length: u32) -> bool {
    let var_id = cell_offset(var_id, index, length);
    get_bool(var_id)
}

pub fn array_set_bool(var_id: u32, index: u32, length: u32, value: bool) {
    let var_id = cell_offset(var_id, index, length);
    set_bool(var_id, value);
}

// Array get32 / set32
pub fn array_get32(var_id: u32, index: u32, length: u32) -> u32 {
    let var_id = cell_offset(var_id, index, length);
    get32(var_id)
}

pub fn array_set32(var_id: u32, index: u32, length: u32, value: u32) {
    let var_id = cell_offset(var_id, index, length);
    set32(var_id, value)
}

// Array get64 / set64
pub fn array_get64(var_id: u32, index: u32, length: u32) -> u64 {
    let var_id = cell_offset(var_id, index, length);
    get64(var_id)
}

pub fn array_set64(var_id: u32, index: u32, length: u32, value: u64) {
    let var_id = cell_offset(var_id, index, length);
    set64(var_id, value)
}

// Array get_amount / set_amount
#[inline]
pub fn array_get_amount(var_id: u32, index: u32, length: u32) -> Amount {
    let value = array_get64(var_id, index, length);

    Amount(value)
}

#[inline]
pub fn array_set_amount(var_id: u32, index: u32, length: u32, value: Amount) {
    let value = value.0;

    array_set64(var_id, index, length, value);
}

#[inline]
fn cell_offset(var_id: u32, index: u32, length: u32) -> u32 {
    let var_id = var_id + index;

    assert!(var_id < length);

    var_id
}

#[inline]
fn int_to_bool(value: u32) -> bool {
    match value {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

#[inline]
fn bool_to_int(value: bool) -> u32 {
    match value {
        true => 1,
        false => 0,
    }
}
