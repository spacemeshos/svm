use svm_types::{
    Address, BytesPrimitive, CodeKind, CodeSection, CtorsSection, DataSection, GasMode, Section,
    Sections, Template, TemplateAddr,
};

/// Returns a predefined, *special* [`Address`] that is used as the deployer for
/// genesis [`svm_types::Template`]'s.
pub fn genesis_templates_deployer_addr() -> Address {
    Address::zeros()
}

pub fn sct() -> (TemplateAddr, Template) {
    let mut sections = Sections::default();

    let code_section = CodeSection::new(
        CodeKind::Wasm,
        include_bytes!("../../../simple-coin-transfer-template/simple_coin_transfer_template.wasm")
            .to_vec(),
        0,
        GasMode::Fixed,
        0,
    );
    let template_addr = svm_types::compute_template_addr(&code_section);

    sections.insert(Section::Code(code_section));
    sections.insert(Section::Data(DataSection::default()));
    sections.insert(Section::Ctors(CtorsSection::default()));
    // TODO: populate noncore sections as well...

    (template_addr, Template::from_sections(sections))
}

#[cfg(test)]
mod test {
    use super::*;
}
