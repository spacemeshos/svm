use crate::{Section, SectionKind};

/// Contains the `Ctors` of the `Template's` Code
#[derive(Debug, Clone, PartialEq)]
pub struct CtorsSection {
    ctors: Vec<String>,
}

impl Default for CtorsSection {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl CtorsSection {
    /// Creates a new Section having input `ctors`
    pub fn new(ctors: Vec<String>) -> Self {
        Self { ctors }
    }

    /// Initializes a new `Section` and reserving room for `capacity` ctors.
    ///
    /// See: `push`
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ctors: Vec::with_capacity(capacity),
        }
    }

    /// Adds a input `ctor` to the Section's ctors.
    pub fn push(&mut self, ctor: String) {
        self.ctors.push(ctor);
    }

    /// Borrows the `ctors` of the Section
    pub fn ctors(&self) -> &[String] {
        &self.ctors
    }

    /// Returns the ctors of the Section as a `Vec` and drops the Section
    pub fn to_vec(self) -> Vec<String> {
        self.ctors
    }
}

impl Section for CtorsSection {
    const KIND: SectionKind = SectionKind::Ctors;
}
