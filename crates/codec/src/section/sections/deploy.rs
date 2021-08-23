//!
//! # `Deploy Section`
//!
//! +------------------+----------------+---------------+-------------+
//! |                  |                |               |             |
//! |  Transaction Id  |     Layer      |   Deployer    |  Template   |
//! |   (32 bytes)     |   (8 bytes)    |   (Address)   |  (Address)  |
//! |                  |                |               |             |
//! +------------------+----------------+---------------+-------------+
//!
//!

use std::io::Cursor;

use svm_types::{Address, DeploySection, Layer, TemplateAddr, TransactionId};

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Field, ParseError, ReadExt, WriteExt};

impl SectionEncoder for DeploySection {
    fn encode(&self, w: &mut Vec<u8>) {
        w.write_bytes_prim(self.tx_id());
        w.write_u64_be(self.layer().0);
        w.write_bytes_prim(self.deployer());
        w.write_bytes_prim(self.template());
    }
}

impl SectionDecoder for DeploySection {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        let tx_id = decode_tx_id(cursor)?;
        let layer = decode_layer(cursor)?;
        let deployer = decode_deployer(cursor)?;
        let template = decode_template(cursor)?;

        let section = DeploySection::new(tx_id, layer, deployer, template);

        Ok(section)
    }
}

fn decode_tx_id(cursor: &mut Cursor<&[u8]>) -> Result<TransactionId, ParseError> {
    let value = cursor.read_bytes_prim();

    value.map_err(|_| ParseError::NotEnoughBytes(Field::TransactionId))
}

fn decode_layer(cursor: &mut Cursor<&[u8]>) -> Result<Layer, ParseError> {
    let layer = cursor.read_u64_be();

    match layer {
        Ok(layer) => Ok(Layer(layer)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Layer)),
    }
}

fn decode_deployer(cursor: &mut Cursor<&[u8]>) -> Result<Address, ParseError> {
    cursor
        .read_bytes_prim()
        .map_err(|_| ParseError::NotEnoughBytes(Field::DeployerAddr))
}

fn decode_template(cursor: &mut Cursor<&[u8]>) -> Result<TemplateAddr, ParseError> {
    cursor
        .read_bytes_prim()
        .map_err(|_| ParseError::NotEnoughBytes(Field::TemplateAddr))
}
