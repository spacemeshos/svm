use std::fmt;
use std::iter::Iterator;

use indexmap::map::Values;
use indexmap::IndexMap;

use super::{
    ApiSection, CodeSection, CtorsSection, DataSection, DeploySection, HeaderSection, SchemaSection,
};

/// A trait to be implemented by each `Section` type.
pub trait SectionLike {
    /// The [`SectionKind`] value associated with the `Section` type implementation.
    const KIND: SectionKind;
}

/// Holds a `Section`
#[derive(Debug, Clone, PartialEq)]
pub enum Section {
    /// A Section of kind `Header`.
    Header(HeaderSection),

    /// A Section of kind `Code`.
    Code(CodeSection),

    /// A Section of kind `Data`.
    Data(DataSection),

    /// A Section of kind `Ctors`.
    Ctors(CtorsSection),

    /// A Section of kind `Schema`.
    Schema(SchemaSection),

    /// A Section of kind `Api`.
    Api(ApiSection),

    /// A Section of kind `Deploy`.
    Deploy(DeploySection),
}

impl Section {
    /// Returns the `SectionKind` associated with the wrapped `Section`
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

    /// Returns the wrapped `HeaderSection`
    ///
    /// # Panics
    ///
    /// Panics if the wrapped `Section` isn't `HeaderSection`
    pub fn as_header(&self) -> &HeaderSection {
        match self {
            Self::Header(section) => section,
            _ => unreachable!(),
        }
    }

    /// Returns the wrapped `CodeSection`
    ///
    /// # Panics
    ///
    /// Panics if the wrapped `Section` isn't `CodeSection`
    pub fn as_code(&self) -> &CodeSection {
        match self {
            Self::Code(section) => section,
            _ => unreachable!(),
        }
    }

    /// Returns the wrapped `DataSection`
    ///
    /// # Panics
    ///
    /// Panics if the wrapped `Section` isn't `DataSection`
    pub fn as_data(&self) -> &DataSection {
        match self {
            Self::Data(section) => section,
            _ => unreachable!(),
        }
    }

    /// Returns the wrapped `CtorsSection`
    ///
    /// # Panics
    ///
    /// Panics if the wrapped `Section` isn't `CtorsSection`
    pub fn as_ctors(&self) -> &CtorsSection {
        match self {
            Self::Ctors(section) => section,
            _ => unreachable!(),
        }
    }

    /// Returns the wrapped `SchemaSection`
    ///
    /// # Panics
    ///
    /// Panics if the wrapped `Section` isn't `SchemaSection`
    pub fn as_schema(&self) -> &SchemaSection {
        match self {
            Self::Schema(section) => section,
            _ => unreachable!(),
        }
    }

    /// Returns the wrapped `ApiSection`
    ///
    /// # Panics
    ///
    /// Panics if the wrapped `Section` isn't `ApiSection`
    pub fn as_api(&self) -> &ApiSection {
        match self {
            Self::Api(section) => section,
            _ => unreachable!(),
        }
    }

    /// Returns the wrapped `DeploySection`
    ///
    /// # Panics
    ///
    /// Panics if the wrapped `Section` isn't `DeploySection`
    pub fn as_deploy(&self) -> &DeploySection {
        match self {
            Self::Deploy(section) => section,
            _ => unreachable!(),
        }
    }
}

impl From<HeaderSection> for Section {
    fn from(section: HeaderSection) -> Self {
        Section::Header(section)
    }
}

impl From<CodeSection> for Section {
    fn from(section: CodeSection) -> Self {
        Section::Code(section)
    }
}

impl From<DataSection> for Section {
    fn from(section: DataSection) -> Self {
        Section::Data(section)
    }
}
impl From<CtorsSection> for Section {
    fn from(section: CtorsSection) -> Self {
        Section::Ctors(section)
    }
}

impl From<SchemaSection> for Section {
    fn from(section: SchemaSection) -> Self {
        Section::Schema(section)
    }
}

impl From<ApiSection> for Section {
    fn from(section: ApiSection) -> Self {
        Section::Api(section)
    }
}

impl From<DeploySection> for Section {
    fn from(section: DeploySection) -> Self {
        Section::Deploy(section)
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind().fmt(f)
    }
}

/// Holds the Kind of `Section`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionKind {
    /// Represents `HeaderSection`
    Header,

    /// Represents `CodeSection`
    Code,

    /// Represents `DataSection`
    Data,

    /// Represents `CtorsSection`
    Ctors,

    /// Represents `SchemaSection`
    Schema,

    /// Represents `ApiSection`
    Api,

    /// Represents `DeploySection`
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

/// Holds a collection of `Section`s
///
/// The `Section`s are indexed by `SectionKind`
/// There may be only one `Section` of each kind.
#[derive(Debug, Clone, PartialEq)]
pub struct Sections {
    inner: IndexMap<SectionKind, Section>,
}

impl Default for Sections {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl Sections {
    /// Reserves room for `capacity` number of [`Section`]s.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity(capacity),
        }
    }

    /// Returns the number of contained [`Section`].
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Inserts a new [`Section`].
    pub fn insert(&mut self, section: Section) {
        self.inner.insert(section.kind(), section);
    }

    /// Returns whether collection has a [`Section`] of `kind`.
    pub fn contains(&self, kind: SectionKind) -> bool {
        self.inner.contains_key(&kind)
    }

    /// Returns the [`Section`] of the requested `kind`.
    ///
    /// # Panics
    ///
    /// Panics if there is no [`Section`] of the requested kind.
    pub fn get(&self, kind: SectionKind) -> &Section {
        self.try_get(kind).unwrap()
    }

    /// Returns the [`Section`] of the requested `kind`.
    ///
    /// Returns `None` if no such [`Section`] exists within `self`.
    pub fn try_get(&self, kind: SectionKind) -> Option<&Section> {
        self.inner.get(&kind)
    }

    /// Takes the [`Section`] of the requested `kind` out of the collection and returns it.
    ///
    /// # Panics
    ///
    /// Panics if there is no `Section` of the requested kind.
    pub fn take(&mut self, kind: SectionKind) -> Section {
        self.try_take(kind).unwrap()
    }

    /// Takes the [`Section`] of the requested `kind` out of `self` and returns it.
    ///
    /// Returns `None` if no such [`Section`] exists within `self`.
    pub fn try_take(&mut self, kind: SectionKind) -> Option<Section> {
        self.inner.remove(&kind)
    }

    /// Returns an iterator over the collection of [`Section`]s.
    pub fn iter<'a>(&'a self) -> SectionsIter<'a> {
        let sections = self.inner.values();

        SectionsIter::new(sections)
    }
}

/// An iterator over a collection of [`Section`]s.
pub struct SectionsIter<'a> {
    sections: Values<'a, SectionKind, Section>,
}

impl<'a> SectionsIter<'a> {
    /// Creates a new Iterator.
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
