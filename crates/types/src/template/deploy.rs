use crate::{DeployerAddr, Layer, Nonce, Section, SectionKind, TemplateAddr, TransactionId};

#[derive(Debug, Clone, PartialEq)]
pub struct DeploySection {
    tx_id: TransactionId,

    layer: Layer,

    nonce: Nonce,

    deployer: DeployerAddr,

    template: TemplateAddr,
}

impl DeploySection {
    pub fn new(
        tx_id: TransactionId,
        layer: Layer,
        nonce: Nonce,
        deployer: DeployerAddr,
        template: TemplateAddr,
    ) -> Self {
        Self {
            tx_id,
            layer,
            nonce,
            deployer,
            template,
        }
    }

    pub fn tx_id(&self) -> &TransactionId {
        &self.tx_id
    }

    pub fn nonce(&self) -> Nonce {
        self.nonce
    }

    pub fn layer(&self) -> Layer {
        self.layer
    }

    pub fn deployer(&self) -> &DeployerAddr {
        &self.deployer
    }

    pub fn template(&self) -> &TemplateAddr {
        &self.template
    }
}

impl Section for DeploySection {
    const KIND: SectionKind = SectionKind::Deploy;
}
