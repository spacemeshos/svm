use svm_types::{
    ApiSection, CodeSection, CtorsSection, DataSection, DeploySection, HeaderSection,
    SchemaSection, SectionKind, SectionWrapper, Sections, Template,
};

pub struct TemplateBuilder {
    sections: Sections,
}

///
/// # Example
///  
/// ```rust
/// use std::io::Cursor;
///
/// use svm_types::Template;
/// use svm_codec::api::builder::DeployTemplateBuilder;
/// use svm_codec::template;
///
/// let layout = vec![5, 10].into();
/// let ctors = vec!["init".to_string()];
///
/// let bytes = TemplateBuilder::new()
///            .with_version(0)
///            .with_name("My Template")
///            .with_code(&[0xC, 0x0, 0xD, 0xE])
///            .with_layout(&layout)
///            .with_ctors(&ctors)
///            .build();
///
/// let mut cursor = Cursor::new(&bytes[..]);
/// let actual = template::decode_deploy_template(&mut cursor).unwrap();
///
/// let expected = Template {
///                  version: 0,
///                  name: "My Template".to_string(),
///                  code: vec![0xC, 0x0, 0xD, 0xE],
///                  layout,
///                  ctors: vec!["init".to_string()]
///                };
///
/// assert_eq!(expected, actual);
/// ```
///

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
