use crate::{DeployerAddr, Layer, Section, SectionKind, TemplateAddr, TransactionId};

/// Stores data related to the deployment of a `Template`
#[derive(Debug, Clone, PartialEq)]
pub struct DeploySection {
    tx_id: TransactionId,

    layer: Layer,

    deployer: DeployerAddr,

    template: TemplateAddr,
}

impl DeploySection {
    /// Creates a new `Section`
    pub fn new(
        tx_id: TransactionId,
        layer: Layer,
        deployer: DeployerAddr,
        template: TemplateAddr,
    ) -> Self {
        Self {
            tx_id,
            layer,
            deployer,
            template,
        }
    }

    /// The `TransactionId` of the `Deploy Template` transaction
    pub fn tx_id(&self) -> &TransactionId {
        &self.tx_id
    }

    /// The `Layer` at which the `Template` has been deployed at
    pub fn layer(&self) -> Layer {
        self.layer
    }

    /// The `Address` of the Account sending the `Deploy Template` transaction
    pub fn deployer(&self) -> &DeployerAddr {
        &self.deployer
    }

    /// The `Address` of the deployed `Template`
    pub fn template(&self) -> &TemplateAddr {
        &self.template
    }
}

impl Section for DeploySection {
    const KIND: SectionKind = SectionKind::Deploy;
}
