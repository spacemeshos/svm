use svm_types::{
    ApiSection, CodeSection, CtorsSection, DataSection, DeploySection, HeaderSection,
    SchemaSection, Section, SectionKind, Sections, Template,
};

/// Builds a `Template`
///
/// # Example
///  
/// ```rust
/// use std::io::Cursor;
///
/// use svm_codec::template;
/// use svm_codec::api::builder::TemplateBuilder;
///
/// use svm_layout::Layout;
/// use svm_types::{Template, CodeKind, CodeSection, CtorsSection, DataSection, GasMode, HeaderSection};
///
/// let code = CodeSection::new(
///      CodeKind::Wasm,
///      vec![0xC0, 0xDE],
///      CodeSection::exec_flags(),
///      GasMode::Fixed,
///      1,
/// );
///
/// let data = DataSection::with_layout(Layout::Fixed(vec![1, 3].into()));
/// let ctors = CtorsSection::new(vec!["init".into(), "start".into()]);
/// let header = HeaderSection::new(2, "My Template".into(), "A few words".into());
///
/// let template = TemplateBuilder::default()
///     .with_code(code)
///     .with_data(data)
///     .with_ctors(ctors)
///     .with_header(header)
///     .build();
/// ```
pub struct TemplateBuilder {
    sections: Sections,
}

impl Default for TemplateBuilder {
    fn default() -> Self {
        Self {
            sections: Sections::default(),
        }
    }
}

impl TemplateBuilder {
    /// Appends `HeaderSection`
    pub fn with_header(mut self, section: HeaderSection) -> Self {
        self.add(section.into());
        self
    }

    /// Appends `CodeSection`
    pub fn with_code(mut self, section: CodeSection) -> Self {
        self.add(section.into());
        self
    }

    /// Appends `DataSection`
    pub fn with_data(mut self, section: DataSection) -> Self {
        self.add(section.into());
        self
    }

    /// Appends `CtorsSection`
    pub fn with_ctors(mut self, section: CtorsSection) -> Self {
        self.add(section.into());
        self
    }

    /// Appends `SchemaSection`
    pub fn with_schema(mut self, section: SchemaSection) -> Self {
        self.add(section.into());
        self
    }

    /// Appends `ApiSection`
    pub fn with_api(mut self, section: ApiSection) -> Self {
        self.add(section.into());
        self
    }

    /// Appends `DeploySection`
    pub fn with_deploy(mut self, section: DeploySection) -> Self {
        self.add(section.into());
        self
    }

    /// Builds a `Template` and drops `self`
    ///
    /// # Panics
    ///
    /// Panics if one of these `Section`s is missing:
    ///
    /// * `CodeSection
    /// * `DataSection
    /// * `CtorsSection
    ///
    /// Also panics is the `DeploySection` exists.
    ///
    /// #### Why is that?
    ///
    /// The `TemplateBuilder` is meant to be used primarily for:
    ///
    /// * Crafting a `Deploy Template` as part of a test
    /// * Crafting a `Deploy Template` transaction prior to dispatching to the network.
    ///
    /// Given a `Template` the best practice to enrich it with a `DeployTemplate` is by calling `Template#set_deploy_template`
    pub fn build(self) -> Template {
        macro_rules! assert_section {
            ($kind:expr) => {{
                if self.sections.contains($kind) == false {
                    panic!("Missing `{}`", $kind)
                }
            }};
        }

        macro_rules! assert_no_section {
            ($kind:expr) => {{
                if self.sections.contains($kind) {
                    panic!(
                        "`{}` can only be added later directly to a `Template`",
                        $kind
                    )
                }
            }};
        }

        assert_section!(SectionKind::Code);
        assert_section!(SectionKind::Data);
        assert_section!(SectionKind::Ctors);
        assert_no_section!(SectionKind::Deploy);

        Template::new(self.sections)
    }

    fn add(&mut self, section: Section) {
        self.sections.insert(section);
    }
}
