use svm_types::{Address, DeploySection, Layer, TemplateAddr, TransactionId};

use crate::{Codec, ParseError, ReadExt, WriteExt};

///
/// # `Deploy Section`
///
/// +------------------+----------------+---------------+-------------+
/// |                  |                |               |             |
/// |  Transaction Id  |     Layer      |   Deployer    |  Template   |
/// |   (32 bytes)     |   (8 bytes)    |   (Address)   |  (Address)  |
/// |                  |                |               |             |
/// +------------------+----------------+---------------+-------------+
///
///
impl Codec for DeploySection {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.tx_id().encode(w);
        (self.layer().0 as u64).encode(w);
        self.deployer().encode(w);
        self.template().encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, ParseError> {
        let tx_id = TransactionId::decode(reader)?;
        let layer = Layer(u64::decode(reader)?);
        let deployer = Address::decode(reader)?;
        let template = TemplateAddr::decode(reader)?;

        Ok(DeploySection::new(tx_id, layer, deployer, template))
    }
}
