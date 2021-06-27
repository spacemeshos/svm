mod api;
mod code;
mod ctors;
mod data;
mod deploy;
mod header;
mod schema;
mod section;

pub use api::ApiSection;
pub use code::{CodeKind, CodeSection};
pub use ctors::CtorsSection;
pub use data::DataSection;
pub use deploy::DeploySection;
pub use header::HeaderSection;
pub use schema::SchemaSection;
pub use section::{Section, SectionKind, SectionWrapper, Sections, SectionsIter};

use svm_layout::FixedLayout;

use std::fmt;

use crate::TemplateAddr;

/// An in-memory representation of a `Template`
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    sections: Sections,
}

impl Template {
    pub fn new(sections: Sections) -> Self {
        Self { sections }
    }

    pub fn sections(&self) -> &Sections {
        &self.sections
    }

    pub fn header_section(&self) -> &HeaderSection {
        let section = self.get(SectionKind::Header);

        section.as_header()
    }

    pub fn code_section(&self) -> &CodeSection {
        let section = self.get(SectionKind::Code);

        section.as_code()
    }

    pub fn code(&self) -> &[u8] {
        let section = self.code_section();

        section.code()
    }

    pub fn data_section(&self) -> &DataSection {
        let section = self.get(SectionKind::Data);

        section.as_data()
    }

    // For now - there can be only a single `Layout` (of type `FixedLayout`)
    // TODO: this method should become obsolete once SVM will support more than one `Layout`s per `Template`
    pub fn fixed_layout(&self) -> &FixedLayout {
        let data = self.data_section();
        let layouts = data.layouts();

        let layout = layouts.first().unwrap();

        layout.as_fixed()
    }

    pub fn ctors_section(&self) -> &CtorsSection {
        let section = self.get(SectionKind::Ctors);

        section.as_ctors()
    }

    pub fn ctors(&self) -> &[String] {
        let section = self.ctors_section();

        section.ctors()
    }

    pub fn is_ctor(&self, func_name: &str) -> bool {
        let ctors = self.ctors();

        ctors
            .iter()
            .find(|ctor| ctor.as_str() == func_name)
            .is_some()
    }

    pub fn schema_section(&self) -> &SchemaSection {
        let section = self.get(SectionKind::Schema);

        section.as_schema()
    }

    pub fn set_deploy_section(&mut self, section: DeploySection) {
        debug_assert!(self.sections.contains(SectionKind::Deploy) == false);

        self.sections.insert(section.into());
    }

    pub fn deploy_section(&self) -> &DeploySection {
        let section = self.get(SectionKind::Deploy);

        section.as_deploy()
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        let section = self.deploy_section();

        section.template()
    }

    pub fn get(&self, kind: SectionKind) -> &SectionWrapper {
        self.sections.get(kind)
    }

    pub fn try_get(&self, kind: SectionKind) -> Option<&SectionWrapper> {
        self.sections.try_get(kind)
    }
}
