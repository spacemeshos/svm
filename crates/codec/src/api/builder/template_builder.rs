use svm_types::{
    ApiSection, CodeSection, CtorsSection, DataSection, DeploySection, HeaderSection,
    SchemaSection, SectionKind, SectionWrapper, Sections, Template,
};

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
    pub fn with_header(mut self, section: HeaderSection) -> Self {
        self.add(section.into());
        self
    }

    pub fn with_code(mut self, section: CodeSection) -> Self {
        self.add(section.into());
        self
    }

    pub fn with_data(mut self, section: DataSection) -> Self {
        self.add(section.into());
        self
    }

    pub fn with_ctors(mut self, section: CtorsSection) -> Self {
        self.add(section.into());
        self
    }

    pub fn with_schema(mut self, section: SchemaSection) -> Self {
        self.add(section.into());
        self
    }

    pub fn with_api(mut self, section: ApiSection) -> Self {
        self.add(section.into());
        self
    }

    pub fn with_deploy(mut self, section: DeploySection) -> Self {
        self.add(section.into());
        self
    }

    pub fn build(mut self) -> Template {
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

    fn add(&mut self, section: SectionWrapper) {
        self.sections.insert(section);
    }
}
