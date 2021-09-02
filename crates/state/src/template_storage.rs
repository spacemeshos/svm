use svm_types::{BytesPrimitive, Sections, TemplateAddr};

use crate::{GlobalState, StorageResult};

pub struct TemplateStorage {
    gs: GlobalState,
    template_addr: TemplateAddr,
}

impl TemplateStorage {
    pub fn new(template_addr: &TemplateAddr, gs: GlobalState) -> Self {
        Self {
            template_addr: template_addr.clone(),
            gs,
        }
    }

    pub fn sections(&self, template_addr: &TemplateAddr) -> StorageResult<Option<Sections>> {
        let core_sections_opt: Option<Sections> =
            self.gs.read_and_decode(&key_core(template_addr))?;
        let noncore_sections_opt: Option<Sections> =
            self.gs.read_and_decode(&key_noncore(template_addr))?;

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

    pub fn set_core(
        &mut self,
        template_addr: &TemplateAddr,
        sections: &Sections,
    ) -> StorageResult<()> {
        self.gs.encode_and_write(sections, &key_core(template_addr));
        Ok(())
    }

    pub fn set_noncore(
        &mut self,
        template_addr: &TemplateAddr,
        sections: &Sections,
    ) -> StorageResult<()> {
        self.gs
            .encode_and_write(sections, &key_noncore(template_addr));
        Ok(())
    }
}

fn key_core(template_addr: &TemplateAddr) -> String {
    format!("templates:{}:core", template_addr.to_string())
}

fn key_noncore(template_addr: &TemplateAddr) -> String {
    format!("templates:{}:noncore", template_addr.to_string())
}
