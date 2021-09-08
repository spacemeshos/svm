use svm_types::{BytesPrimitive, Sections, TemplateAddr};

use crate::{GlobalState, StorageResult};

/// A [`GlobalState`] wrapper, enriched with utility methods to access and
/// modify [`Template`](svm_types::Template) data.
pub struct TemplateStorage {
    /// The internal [`GlobalState`] instance used to access the database layer.
    pub gs: GlobalState,

    template_addr: TemplateAddr,
}

impl TemplateStorage {
    /// Creates a new [`TemplateStorage`] from the given address and
    /// [`GlobalState`] instance.
    pub fn new(template_addr: &TemplateAddr, gs: GlobalState) -> Self {
        Self {
            template_addr: template_addr.clone(),
            gs,
        }
    }

    /// Reads, decodes and finally returns all [`Sections`] of `self`.
    pub fn sections(&self) -> StorageResult<Option<Sections>> {
        let core_sections_opt: Option<Sections> =
            self.gs.read_and_decode(&key_core(&self.template_addr))?;
        let noncore_sections_opt: Option<Sections> =
            self.gs.read_and_decode(&key_noncore(&self.template_addr))?;

        match (core_sections_opt, noncore_sections_opt) {
            (Some(mut sections), Some(noncore)) => {
                for s in noncore.iter().cloned() {
                    sections.insert(s);
                }
                Ok(Some(sections))
            }
            _ => return Ok(None),
        }
    }

    /// Overwrites the "core" (mandatory) [`Sections`] associated with
    /// `self`.
    pub fn set_core(&mut self, sections: &Sections) -> StorageResult<()> {
        let key = key_core(&self.template_addr);
        self.gs.encode_and_write(sections, &key);

        Ok(())
    }

    /// Overwrites the "non-core" (optional) [`Sections`] associated with
    /// `self`.
    pub fn set_noncore(&mut self, sections: &Sections) -> StorageResult<()> {
        let key = key_noncore(&self.template_addr);
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
