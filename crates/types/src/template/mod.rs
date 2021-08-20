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

use svm_layout::FixedLayout;

use crate::TemplateAddr;

/// An in-memory representation of a `Template`.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    // Mandatory sections.
    pub code_section: CodeSection,
    pub data_section: DataSection,
    pub ctors_section: CtorsSection,
    // Optional sections.
    pub header_section: Option<HeaderSection>,
    pub deploy_section: Option<DeploySection>,
    pub api_section: Option<ApiSection>,
    pub schema_section: Option<SchemaSection>,
}

impl Template {
    /// Creates a new [`Template`] only with the mandatory sections.
    pub fn new(
        code_section: CodeSection,
        data_section: DataSection,
        ctors_section: CtorsSection,
    ) -> Self {
        Self {
            code_section,
            ctors_section,
            data_section,
            header_section: None,
            deploy_section: None,
            api_section: None,
            schema_section: None,
        }
    }

    /// Adds a [`HeaderSection`] to `self`.
    pub fn with_header(mut self, header: HeaderSection) -> Self {
        self.header_section = Some(header);
        self
    }

    /// Creates a new [`Template`] from a collection of [`Sections`].
    ///
    /// # Panics
    ///
    /// Panics if and only if `sections` does *not* contain all mandatory
    /// [`Template`] sections.
    pub fn from_sections(sections: Sections) -> Self {
        let code_section = sections.get(SectionKind::Code).as_code().clone();
        let ctors_section = sections.get(SectionKind::Ctors).as_ctors().clone();
        let data_section = sections.get(SectionKind::Data).as_data().clone();

        let header_section = sections
            .try_get(SectionKind::Header)
            .map(Section::as_header)
            .cloned();
        let deploy_section = sections
            .try_get(SectionKind::Deploy)
            .map(Section::as_deploy)
            .cloned();
        let api_section = sections
            .try_get(SectionKind::Api)
            .map(Section::as_api)
            .cloned();
        let schema_section = sections
            .try_get(SectionKind::Schema)
            .map(Section::as_schema)
            .cloned();

        Self {
            code_section,
            ctors_section,
            data_section,
            header_section,
            deploy_section,
            api_section,
            schema_section,
        }
    }

    /// Creates, populates and finally returns a [`Sections`] of `self`.
    pub fn sections(&self) -> Sections {
        let mut sections = Sections::default();

        sections.insert(Section::Code(self.code_section.clone()));
        sections.insert(Section::Data(self.data_section.clone()));
        sections.insert(Section::Ctors(self.ctors_section.clone()));

        if let Some(ref s) = self.header_section {
            sections.insert(Section::Header(s.clone()));
        }
        if let Some(ref s) = self.deploy_section {
            sections.insert(Section::Deploy(s.clone()));
        }
        if let Some(ref s) = self.api_section {
            sections.insert(Section::Api(s.clone()));
        }
        if let Some(ref s) = self.schema_section {
            sections.insert(Section::Schema(s.clone()));
        }

        sections
    }

    /// Borrows the `Template's` code (a blob)
    ///
    /// # Panics
    ///
    /// Panics if there is no `Code Section`
    pub fn code(&self) -> &[u8] {
        self.code_section.code()
    }

    /// Returns an immutable borrow of `self`'s [`FixedLayout`].
    pub fn fixed_layout(&self) -> &FixedLayout {
        // See <https://github.com/spacemeshos/svm/issues/281>.
        let data = &self.data_section;
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
        &self.ctors_section
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

        ctors
            .iter()
            .find(|ctor| ctor.as_str() == func_name)
            .is_some()
    }

    /// Borrows the `Schema Section`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Schema Section`
    pub fn schema_section(&self) -> &SchemaSection {
        self.schema_section.as_ref().unwrap()
    }

    /// Returns the `Address` of a deployed `Template`
    ///
    /// # Panics
    ///
    /// Panics if there is no `Deploy Section`
    pub fn template_addr(&self) -> &TemplateAddr {
        self.deploy_section.as_ref().unwrap().template()
    }
}
