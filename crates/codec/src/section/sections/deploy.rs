//!
//! # `Deploy Section`
//!
//! +------------------+----------------+--------------+---------------+-------------+
//! |                  |                |              |               |             |
//! |  Transaction Id  |     Layer      |    Nonce     |   Deployer    |  Template   |
//! |   (32 bytes)     |   (8 bytes)    |   (8 bytes)  |   (Address)   |  (Address)  |
//! |                  |                |              |               |             |
//! +------------------+----------------+--------------+---------------+-------------+
//!
//!

use std::io::Cursor;

use svm_types::{
    DeploySection, DeployerAddr, Layer, Nonce, Section, SectionKind, TemplateAddr, TransactionId,
};

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Field, ParseError, ReadExt, WriteExt};

impl SectionEncoder for DeploySection {
    fn encode(&self, w: &mut Vec<u8>) {
        encode_tx_id(self.tx_id(), w);
        encode_layer(self.layer(), w);
        encode_nonce(self.nonce(), w);
        encode_deployer(self.deployer(), w);
        encode_template(self.template(), w);
    }
}

fn encode_tx_id(tx_id: &TransactionId, w: &mut Vec<u8>) {
    w.write_tx_id(tx_id);
}

fn encode_layer(layer: Layer, w: &mut Vec<u8>) {
    w.write_u64_be(layer.0);
}

fn encode_nonce(nonce: Nonce, w: &mut Vec<u8>) {
    w.write_u64_be(nonce.0);
}

fn encode_deployer(deployer: &DeployerAddr, w: &mut Vec<u8>) {
    w.write_address(deployer.inner());
}

fn encode_template(template: &TemplateAddr, w: &mut Vec<u8>) {
    w.write_address(template.inner());
}

impl SectionDecoder for DeploySection {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        let tx_id = decode_tx_id(cursor)?;
        let layer = decode_layer(cursor)?;
        let nonce = decode_nonce(cursor)?;
        let deployer = decode_deployer(cursor)?;
        let template = decode_template(cursor)?;

        let section = DeploySection::new(tx_id, layer, nonce, deployer, template);

        Ok(section)
    }
}

fn decode_tx_id(cursor: &mut Cursor<&[u8]>) -> Result<TransactionId, ParseError> {
    let value = cursor.read_tx_id();

    value.map_err(|_| ParseError::NotEnoughBytes(Field::TransactionId))
}

fn decode_layer(cursor: &mut Cursor<&[u8]>) -> Result<Layer, ParseError> {
    let layer = cursor.read_u64_be();

    match layer {
        Ok(layer) => Ok(Layer(layer)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Layer)),
    }
}

fn decode_nonce(cursor: &mut Cursor<&[u8]>) -> Result<Nonce, ParseError> {
    let nonce = cursor.read_u64_be();

    match nonce {
        Ok(nonce) => Ok(Nonce(nonce)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Nonce)),
    }
}

fn decode_deployer(cursor: &mut Cursor<&[u8]>) -> Result<DeployerAddr, ParseError> {
    let addr = cursor.read_address();

    match addr {
        Ok(addr) => Ok(addr.into()),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::DeployerAddr)),
    }
}

fn decode_template(cursor: &mut Cursor<&[u8]>) -> Result<TemplateAddr, ParseError> {
    let addr = cursor.read_address();

    match addr {
        Ok(addr) => Ok(addr.into()),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::TemplateAddr)),
    }
}
