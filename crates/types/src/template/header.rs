use crate::{DeployerAddr, Section, SectionKind};

#[derive(Debug, Clone, PartialEq)]
pub struct HeaderSection {
    code_version: u32,

    name: String,

    desc: String,
}

impl HeaderSection {
    pub fn new(code_version: u32, name: String, desc: String) -> Self {
        Self {
            code_version,
            name,
            desc,
        }
    }

    pub fn code_version(&self) -> u32 {
        self.code_version
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }
}

impl Section for HeaderSection {
    const KIND: SectionKind = SectionKind::Header;
}
