use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::fmt;
use std::iter::Iterator;

use super::{
    ApiSection, CodeSection, CtorsSection, DataSection, DeploySection, HeaderSection, SchemaSection,
};

#[derive(Clone)]
pub enum Section {
    Header(HeaderSection),
    Code(CodeSection),
    Data(DataSection),
    Ctors(CtorsSection),
    Schema(SchemaSection),
    Api(ApiSection),
    Deploy(DeploySection),
}

impl Section {
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

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Header(..) => write!(f, "Header Section"),
            Self::Code(..) => write!(f, "Code Section"),
            Self::Data(..) => write!(f, "Data Section"),
            Self::Ctors(..) => write!(f, "Ctors Section"),
            Self::Schema(..) => write!(f, "Schema Section"),
            Self::Api(..) => write!(f, "API Section"),
            Self::Deploy(..) => write!(f, "Deploy Section"),
        }
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

#[derive(Clone)]
pub struct Sections {
    inner: HashMap<SectionKind, Section>,
}

impl Sections {
    pub fn insert(&mut self, section: Section) {
        self.inner.insert(section.kind(), section);
    }

    pub fn contains(&self, kind: SectionKind) -> bool {
        self.inner.contains_key(&kind)
    }

    pub fn get(&self, kind: SectionKind) -> &Section {
        self.inner.get(&kind).unwrap()
    }

    pub fn try_get(&self, kind: SectionKind) -> Option<&Section> {
        self.inner.get(&kind)
    }

    pub fn iter<'a>(&'a self) -> SectionsIter<'a> {
        let sections = self.inner.values();

        SectionsIter { sections }
    }
}

pub struct SectionsIter<'a> {
    sections: Values<'a, SectionKind, Section>,
}

impl<'a> SectionsIter<'a> {
    fn new(sections: Values<'a, SectionKind, Section>) -> Self {
        Self { sections }
    }
}

impl<'a> Iterator for SectionsIter<'a> {
    type Item = &'a Section;

    fn next(&mut self) -> Option<Self::Item> {
        self.sections.next()
    }
}
