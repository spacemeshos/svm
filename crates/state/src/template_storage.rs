use svm_types::{BytesPrimitive, Sections, TemplateAddr};

use crate::{GlobalState, StorageResult};

/// A [`GlobalState`] wrapper, enriched with utility methods to access and
/// modify [`Template`](svm_types::Template) data.
pub struct TemplateStorage {
    /// The internal [`GlobalState`] instance used to access the database layer.
    pub gs: GlobalState,

    /// The [`TemplateAddr`] of `self`.
    pub addr: TemplateAddr,
}

impl TemplateStorage {
    /// Loads a [`TemplateStorage`] from the given address and
    /// [`GlobalState`] instance.
    pub fn load(gs: GlobalState, template_addr: &TemplateAddr) -> StorageResult<Self> {
        Ok(Self {
            gs,
            addr: template_addr.clone(),
        })
    }

    /// Saves a [`TemplateStorage`] at the given address and
    /// on the given [`GlobalState`] instance.
    pub fn create(
        mut gs: GlobalState,
        template_addr: &TemplateAddr,
        core_sections: Sections,
        noncore_sections: Sections,
    ) -> StorageResult<Self> {
        gs.encode_and_write(&core_sections, &key_core(&template_addr));
        gs.encode_and_write(&noncore_sections, &key_noncore(&template_addr));

        Ok(Self {
            gs,
            addr: template_addr.clone(),
        })
    }

    /// Reads, decodes and finally returns all [`Sections`] of `self`.
    pub fn sections(&self) -> StorageResult<Sections> {
        read_sections(&self.gs, &self.addr)
    }

    /// Overwrites the "core" (mandatory) [`Sections`] associated with
    /// `self`.
    pub fn set_core(&mut self, sections: &Sections) -> StorageResult<()> {
        let key = key_core(&self.addr);
        self.gs.encode_and_write(sections, &key);

        Ok(())
    }

    /// Overwrites the "non-core" (optional) [`Sections`] associated with
    /// `self`.
    pub fn set_noncore(&mut self, sections: &Sections) -> StorageResult<()> {
        let key = key_noncore(&self.addr);
        self.gs.encode_and_write(sections, &key);

        Ok(())
    }
}

fn key_core(template_addr: &TemplateAddr) -> String {
    format!("templates:{}:core", template_addr.to_string())
}

fn key_noncore(template_addr: &TemplateAddr) -> String {
    format!("templates:{}:noncore", template_addr.to_string())
}

fn read_sections(gs: &GlobalState, addr: &TemplateAddr) -> StorageResult<Sections> {
    let mut sections = gs.read_and_decode::<Sections>(&key_core(addr))?;
    let noncore = gs.read_and_decode::<Sections>(&key_noncore(addr))?;

    for s in noncore.iter().cloned() {
        sections.insert(s);
    }
    Ok(sections)
}

#[cfg(test)]
mod test {
    use svm_layout::{FixedLayout, Layout};
    use svm_types::{CodeSection, CtorsSection, DataSection, SectionKind, Sections, Template};

    use super::*;

    fn fixed_layout() -> FixedLayout {
        FixedLayout::from_byte_sizes(0, &[10, 20, 4, 30, 64, 31, 100, 4, 8, 8])
    }

    fn new_template(gs: &GlobalState) -> TemplateAddr {
        let template_addr = TemplateAddr::repeat(0x80);

        let code_section = CodeSection::new(
            svm_types::CodeKind::Wasm,
            vec![],
            0,
            svm_types::GasMode::Fixed,
            0,
        );
        let data_section = DataSection::with_layout(Layout::Fixed(fixed_layout()));
        let ctors_section = CtorsSection::new(vec![]);

        let core_sections = Template::new(code_section, data_section, ctors_section)
            .sections()
            .clone();
        let noncore_sections = Sections::with_capacity(0);

        TemplateStorage::create(gs.clone(), &template_addr, core_sections, noncore_sections)
            .unwrap();

        template_addr
    }

    #[test]
    fn create_then_load() {
        let gs = GlobalState::in_memory();

        let template_addr = new_template(&gs);

        let template_storage = TemplateStorage::load(gs, &template_addr).unwrap();

        assert!(template_storage
            .sections()
            .unwrap()
            .get(SectionKind::Code)
            .as_code()
            .code()
            .is_empty());
    }
}
