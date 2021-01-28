use std::io::{Cursor, Read};

use super::gas;
use crate::{Field, ParseError, ReadExt, WriteExt};

use svm_types::gas::MaybeGas;
use svm_types::receipt::{Log, Receipt, ReceiptError};
use svm_types::{Address, State};

/// Encoders

pub(crate) fn encode_is_success(receipt: &Receipt, w: &mut Vec<u8>) {
    let nib = if receipt.is_success() {
        w.push(1);
    } else {
        w.push(0);
    };
}

/// Decoders

pub(crate) fn decode_is_success(cursor: &mut Cursor<&[u8]>) -> Result<u8, ParseError> {
    cursor
        .read_byte()
        .map_err(|_| ParseError::NotEnoughBytes(Field::ReceiptStatus))
}
