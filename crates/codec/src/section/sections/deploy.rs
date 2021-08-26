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
use crate::{Codec, ParseError, ReadExt};

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
    Ok(TransactionId::decode(cursor)?)
}

fn decode_layer(cursor: &mut impl ReadExt) -> Result<Layer, ParseError> {
    Ok(Layer(u64::decode(cursor)?))
}

fn decode_deployer(cursor: &mut impl ReadExt) -> Result<Address, ParseError> {
    Ok(Address::decode(cursor)?)
}

fn decode_template(cursor: &mut impl ReadExt) -> Result<TemplateAddr, ParseError> {
    Ok(TemplateAddr::decode(cursor)?)
}
