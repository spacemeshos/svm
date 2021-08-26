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

use svm_types::{Address, DeploySection, Layer, TemplateAddr, TransactionId};

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Codec, Field, ParseError, ReadExt};

impl SectionEncoder for DeploySection {
    fn encode(&self, w: &mut Vec<u8>) {
        self.tx_id().encode(w);
        (self.layer().0 as u64).encode(w);
        self.deployer().encode(w);
        self.template().encode(w);
    }
}

impl SectionDecoder for DeploySection {
    fn decode(cursor: &mut impl ReadExt) -> Result<Self, ParseError> {
        let tx_id = decode_tx_id(cursor)?;
        let layer = decode_layer(cursor)?;
        let deployer = decode_deployer(cursor)?;
        let template = decode_template(cursor)?;

        let section = DeploySection::new(tx_id, layer, deployer, template);

        Ok(section)
    }
}

fn decode_tx_id(cursor: &mut impl ReadExt) -> Result<TransactionId, ParseError> {
    let value = TransactionId::decode(cursor);

    value.map_err(|_| ParseError::Eof(Field::TransactionId.to_string()))
}

fn decode_layer(cursor: &mut impl ReadExt) -> Result<Layer, ParseError> {
    let layer = u64::decode(cursor);

    match layer {
        Ok(layer) => Ok(Layer(layer)),
        Err(..) => Err(ParseError::Eof(Field::Layer.to_string())),
    }
}

fn decode_deployer(cursor: &mut impl ReadExt) -> Result<Address, ParseError> {
    Address::decode(cursor).map_err(|_| ParseError::Eof(Field::DeployerAddr.to_string()))
}

fn decode_template(cursor: &mut impl ReadExt) -> Result<TemplateAddr, ParseError> {
    TemplateAddr::decode(cursor).map_err(|_| ParseError::Eof(Field::TemplateAddr.to_string()))
}
