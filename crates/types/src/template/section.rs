use std::fmt;
use std::iter::Iterator;

use indexmap::{map::Values, IndexMap};

use super::{
    ApiSection, CodeSection, CtorsSection, DataSection, DeploySection, HeaderSection, SchemaSection,
};

pub trait Section {
    const KIND: SectionKind;
}

#[derive(Debug, Clone, PartialEq)]
pub enum SectionWrapper {
    Header(HeaderSection),
    Code(CodeSection),
    Data(DataSection),
    Ctors(CtorsSection),
    Schema(SchemaSection),
    Api(ApiSection),
    Deploy(DeploySection),
}

impl SectionWrapper {
    pub fn kind(&self) -> SectionKind {
        match *self {
            Self::Header(..) => SectionKind::Header,
            Self::Code(..) => SectionKind::Code,
            Self::Data(..) => SectionKind::Data,
            Self::Ctors(..) => SectionKind::Ctors,
            Self::Schema(..) => SectionKind::Schema,
            Self::Api(..) => SectionKind::Api,
            Self::Deploy(..) => SectionKind::Deploy,
        }
    }

    pub fn as_header(&self) -> &HeaderSection {
        match self {
            Self::Header(section) => section,
            _ => unreachable!(),
        }
    }

    pub fn as_code(&self) -> &CodeSection {
        match self {
            Self::Code(section) => section,
            _ => unreachable!(),
        }
    }

    pub fn as_data(&self) -> &DataSection {
        match self {
            Self::Data(section) => section,
            _ => unreachable!(),
        }
    }

    pub fn as_ctors(&self) -> &CtorsSection {
        match self {
            Self::Ctors(section) => section,
            _ => unreachable!(),
        }
    }

    pub fn as_schema(&self) -> &SchemaSection {
        match self {
            Self::Schema(section) => section,
            _ => unreachable!(),
        }
    }

    pub fn as_api(&self) -> &ApiSection {
        match self {
            Self::Api(section) => section,
            _ => unreachable!(),
        }
    }

    pub fn as_deploy(&self) -> &DeploySection {
        match self {
            Self::Deploy(section) => section,
            _ => unreachable!(),
        }
    }
}

impl From<HeaderSection> for SectionWrapper {
    fn from(section: HeaderSection) -> Self {
        SectionWrapper::Header(section)
    }
}

impl From<CodeSection> for SectionWrapper {
    fn from(section: CodeSection) -> Self {
        SectionWrapper::Code(section)
    }
}

impl From<DataSection> for SectionWrapper {
    fn from(section: DataSection) -> Self {
        SectionWrapper::Data(section)
    }
}
impl From<CtorsSection> for SectionWrapper {
    fn from(section: CtorsSection) -> Self {
        SectionWrapper::Ctors(section)
    }
}

impl From<SchemaSection> for SectionWrapper {
    fn from(section: SchemaSection) -> Self {
        SectionWrapper::Schema(section)
    }
}

impl From<ApiSection> for SectionWrapper {
    fn from(section: ApiSection) -> Self {
        SectionWrapper::Api(section)
    }
}

impl From<DeploySection> for SectionWrapper {
    fn from(section: DeploySection) -> Self {
        SectionWrapper::Deploy(section)
    }
}

impl fmt::Display for SectionWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind().fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionKind {
    Header,
    Code,
    Data,
    Ctors,
    Schema,
    Api,
    Deploy,
}

impl fmt::Display for SectionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header => write!(f, "Header Section"),
            Self::Code => write!(f, "Code Section"),
            Self::Data => write!(f, "Data Section"),
            Self::Ctors => write!(f, "Ctors Section"),
            Self::Schema => write!(f, "Schema Section"),
            Self::Api => write!(f, "API Section"),
            Self::Deploy => write!(f, "Deploy Section"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sections {
    inner: IndexMap<SectionKind, SectionWrapper>,
}

impl Default for Sections {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl Sections {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, section: SectionWrapper) {
        self.inner.insert(section.kind(), section);
    }

    pub fn contains(&self, kind: SectionKind) -> bool {
        self.inner.contains_key(&kind)
    }

    pub fn get(&self, kind: SectionKind) -> &SectionWrapper {
        self.try_get(kind).unwrap()
    }

    pub fn take(&mut self, kind: SectionKind) -> SectionWrapper {
        self.try_take(kind).unwrap()
    }

    pub fn try_get(&self, kind: SectionKind) -> Option<&SectionWrapper> {
        self.inner.get(&kind)
    }

    pub fn try_take(&mut self, kind: SectionKind) -> Option<SectionWrapper> {
        self.inner.remove(&kind)
    }

    pub fn iter<'a>(&'a self) -> SectionsIter<'a> {
        let sections = self.inner.values();

        SectionsIter { sections }
    }
}

pub struct SectionsIter<'a> {
    sections: Values<'a, SectionKind, SectionWrapper>,
}

impl<'a> SectionsIter<'a> {
    fn new(sections: Values<'a, SectionKind, SectionWrapper>) -> Self {
        Self { sections }
    }
}

impl<'a> Iterator for SectionsIter<'a> {
    type Item = &'a SectionWrapper;

    fn next(&mut self) -> Option<Self::Item> {
        self.sections.next()
    }
}
