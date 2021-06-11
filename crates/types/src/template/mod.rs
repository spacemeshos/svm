mod api;
mod code;
mod ctors;
mod data;
mod deploy;
mod header;
mod schema;
mod section;

pub use api::ApiSection;
pub use code::{ByteCodeKind, CodeSection};
pub use ctors::CtorsSection;
pub use data::DataSection;
pub use deploy::DeploySection;
pub use header::HeaderSection;
pub use schema::SchemaSection;
pub use section::{Section, SectionKind, Sections, SectionsIter};

use std::fmt;

/// An in-memory representation of a `Template`
#[allow(missing_docs)]
#[derive(Clone)]
pub struct Template {
    sections: Sections,
}

impl Template {
    pub fn header(&self) -> &HeaderSection {
        let section = self.get(SectionKind::Header);

        section.as_header()
    }

    pub fn code(&self) -> &CodeSection {
        let section = self.get(SectionKind::Code);

        section.as_code()
    }

    pub fn data(&self) -> &DataSection {
        let section = self.get(SectionKind::Data);

        section.as_data()
    }

    pub fn ctors(&self) -> &CtorsSection {
        let section = self.get(SectionKind::Ctors);

        section.as_ctors()
    }

    pub fn schema(&self) -> &SchemaSection {
        let section = self.get(SectionKind::Schema);

        section.as_schema()
    }

    pub fn deploy(&self) -> &DeploySection {
        let section = self.get(SectionKind::Deploy);

        section.as_deploy()
    }

    pub fn get(&self, kind: SectionKind) -> &Section {
        self.sections.get(kind)
    }

    pub fn try_get(&self, kind: SectionKind) -> Option<&Section> {
        self.sections.try_get(kind)
    }
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Template").finish()
    }
}

fn fmt_code(code: &[u8]) -> String {
    let n = std::cmp::min(code.len(), 4);

    format!("{:?}", &code[0..n])
}
