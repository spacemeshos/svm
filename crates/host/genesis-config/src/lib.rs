use std::collections::HashMap;
use svm_layout::{FixedLayout, Layout};
use svm_types::{
    Address, BytesPrimitive, CodeKind, CodeSection, CtorsSection, DataSection, GasMode, Section,
    Sections, Template, TemplateAddr,
};

/// Information about templates already deployed at genesis.
#[derive(Debug)]
pub struct GenesisConfig {
    /// A list of [`Template`]'s to deploy at genesis, together with their
    /// respective addresses.
    pub templates: HashMap<TemplateAddr, Template>,
}

impl GenesisConfig {
    /// Creates a new [`GenesisConfig`] with the templates that are available at
    /// genesis on the Spacemesh mainnet.
    pub fn mainnet() -> Self {
        let mut genesis = Self {
            templates: HashMap::new(),
        };
        let sct = sct();
        genesis.templates.insert(sct.0, sct.1);
        genesis
    }
}

/// Returns a predefined, *special* [`Address`] that is used as the deployer for
/// genesis [`svm_types::Template`]'s.
pub fn genesis_templates_deployer_addr() -> Address {
    Address::zeros()
}

pub fn sct() -> (TemplateAddr, Template) {
    let mut sections = Sections::default();

    let code_section = CodeSection::new(
        CodeKind::Wasm,
        include_bytes!(
            "../../../../simple-coin-transfer-template/simple_coin_transfer_template.wasm"
        )
        .to_vec(),
        0,
        GasMode::Fixed,
        0,
    );
    let template_addr = svm_types::compute_template_addr(&code_section);

    sections.insert(Section::Code(code_section));
    sections.insert(Section::Data(DataSection::with_layout(Layout::Fixed(
        FixedLayout::default(),
    ))));
    sections.insert(Section::Ctors(CtorsSection::default()));
    // TODO: populate noncore sections as well...

    (template_addr, Template::from_sections(sections))
}

#[cfg(test)]
mod test {}
