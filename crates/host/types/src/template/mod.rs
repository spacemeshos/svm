//! Generally, SVM needs to retrieves different pieces of data in different contexts.
//! We can roughly categorize the content as Core and Non-Core.
//!
//! It's pivotal to be able to fetch only the required data as fast as we can.
//! Secondly, it's crucial to make the format of a Template easy to evolve in the future.
//!
//! The solution introduced in this PR is called the Template Sections.
//! Instead of looking at a Template as a single unit - from now on, we'll consider a Template
//! as a collection of independent Section. Each Section will serve as single purpose and belong to
//! a either the Core or Non-Core category.
//!
//! The Template is an evolving entity. There is the pre-deploy stage and post-deploy stages.
//! Instead of having something such as PreDeployTemplate and PostDeployTemplate in the code base - deploying a Template means adding a Deploy Section to a collection of Section.
//!
//! If in the future we'll add a removal feature for a Template - we could come up with a new Removal Section or similar.
//!
//! Moving into the Sections design, we can now go further and store groups of Sections in different indexes very naturally.
//! Upon fetch, we should query the required indexes and get the corresponding Sections back.
//!
//! This PR also implements a skip Section functionality when loading data.
//!
//! Each Section is prefixed with a Section Preview, so when decoding the raw fetched data, we can ask to skip Sections that we don't want and only decode the ones we want.

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
pub use section::{Section, SectionKind, SectionLike, Sections, SectionsIter};

use svm_hash::{Blake3Hasher, Hasher};
use svm_layout::FixedLayout;

use crate::{BytesPrimitive, TemplateAddr};

/// An in-memory representation of a `Template`
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    sections: Sections,
}

impl Template {
    /// Creates a new [`Template`] only with the mandatory sections.
    pub fn new(
        code_section: CodeSection,
        data_section: DataSection,
        ctors_section: CtorsSection,
    ) -> Self {
        let mut sections = Sections::default();
        sections.insert(Section::Code(code_section));
        sections.insert(Section::Data(data_section));
        sections.insert(Section::Ctors(ctors_section));

        Self::from_sections(sections)
    }

    /// Creates a new `Template`
    ///
    /// # Panics
    ///
    /// Panics if and only if `sections` does *not* contain all mandatory
    /// [`Template`] sections.
    pub fn from_sections(sections: Sections) -> Self {
        Self { sections }
    }

    /// Adds, removes or replaces a [`HeaderSection`] in `self`.
    pub fn with_header(mut self, header: Option<HeaderSection>) -> Self {
        if let Some(header) = header {
            self.sections.insert(Section::Header(header));
        }

        self
    }

    /// Adds, removes or replaces a [`DeploySection`] in `self`.
    pub fn with_deploy(mut self, deploy: Option<DeploySection>) -> Self {
        if let Some(deploy) = deploy {
            self.sections.insert(Section::Deploy(deploy));
        }

        self
    }

    /// Borrows the `Sections` of the `Template`
    pub fn sections(&self) -> &Sections {
        &self.sections
    }

    /// Borrows the `Header Section`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Header Section`
    pub fn header_section(&self) -> &HeaderSection {
        let section = self.get(SectionKind::Header);
        section.as_header()
    }

    /// Borrows the `Code Section`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Code Section`
    pub fn code_section(&self) -> &CodeSection {
        let section = self.get(SectionKind::Code);
        section.as_code()
    }

    /// Borrows the `Template's` code (a blob)
    ///
    /// # Panics
    ///
    /// Panics if there is no `Code Section`
    pub fn code(&self) -> &[u8] {
        let section = self.code_section();
        section.code()
    }

    /// Borrows the `Data Section`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Data Section`
    pub fn data_section(&self) -> &DataSection {
        let section = self.get(SectionKind::Data);
        section.as_data()
    }

    /// Returns an immutable borrow of `self`'s [`FixedLayout`].
    pub fn fixed_layout(&self) -> &FixedLayout {
        // See <https://github.com/spacemeshos/svm/issues/281>.
        let data = self.data_section();
        let layouts = data.layouts();

        let layout = layouts.first().unwrap();
        layout.as_fixed()
    }

    /// Borrows the `Ctors Section`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Ctors Section`
    pub fn ctors_section(&self) -> &CtorsSection {
        let section = self.get(SectionKind::Ctors);
        section.as_ctors()
    }

    /// Returns the `Ctors` of the `Template`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Ctors Section`
    pub fn ctors(&self) -> &[String] {
        let section = self.ctors_section();
        section.ctors()
    }

    /// Returns whether `Template` has a `ctor` equals `func_name`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Ctors Section`
    pub fn is_ctor(&self, func_name: &str) -> bool {
        let ctors = self.ctors();
        ctors.iter().any(|ctor| ctor.as_str() == func_name)
    }

    /// Borrows the `Schema Section`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Schema Section`
    pub fn schema_section(&self) -> &SchemaSection {
        let section = self.get(SectionKind::Schema);
        section.as_schema()
    }

    /// Sets the `DeploySection` to a `Template`
    pub fn set_deploy_section(&mut self, section: DeploySection) {
        debug_assert!(self.sections.contains(SectionKind::Deploy) == false);

        self.sections.insert(section.into());
    }

    /// Borrows the `Deploy Section`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Deploy Section`
    pub fn deploy_section(&self) -> &DeploySection {
        let section = self.get(SectionKind::Deploy);
        section.as_deploy()
    }

    /// Returns the `Address` of a deployed `Template`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Deploy Section`
    pub fn template_addr(&self) -> &TemplateAddr {
        let section = self.deploy_section();
        section.template()
    }

    /// Borrows the `Section` of the requested `SectionKind`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Section` having the requested `SectionKind`
    pub fn get(&self, kind: SectionKind) -> &Section {
        self.sections.get(kind)
    }

    /// Borrows the `Section` of the requested `SectionKind`
    ///
    /// Returns `None` when there is no `Section` of the specified `SectionKind`
    pub fn try_get(&self, kind: SectionKind) -> Option<&Section> {
        self.sections.try_get(kind)
    }

    /// Returns whether Section exists
    pub fn contains(&self, kind: SectionKind) -> bool {
        self.sections.try_get(kind).is_some()
    }
}

/// Calculates the address of a newly deployed [`Template`].
pub fn compute_template_addr(code_section: &CodeSection) -> TemplateAddr {
    let hash = Blake3Hasher::hash(code_section.code());

    TemplateAddr::new(&hash[..TemplateAddr::N])
}
