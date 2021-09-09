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

    /// Loads a [`TemplateStorage`] from the given address and
    /// [`GlobalState`] instance.
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
