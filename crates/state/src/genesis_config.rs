use std::collections::HashMap;

use svm_types::{Template, TemplateAddr};

#[derive(Debug)]
pub struct GenesisConfig {
    pub templates: HashMap<TemplateAddr, Template>,
}

impl GenesisConfig {
    /// Creates a new [`GenesisConfig`] with the templates that are available at
    /// genesis on the Spacemesh mainnet.
    pub fn mainnet() -> Self {
        let mut genesis = Self {
            templates: HashMap::new(),
        };
        let sct = svm_genesis_templates::sct();
        genesis.templates.insert(sct.0, sct.1);
        genesis
    }
}
