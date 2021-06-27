use crate::{Section, SectionKind};

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
    pub fn new(ctors: Vec<String>) -> Self {
        Self { ctors }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ctors: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, ctor: String) {
        self.ctors.push(ctor);
    }

    pub fn ctors(&self) -> &[String] {
        &self.ctors
    }

    pub fn to_vec(self) -> Vec<String> {
        self.ctors
    }

    pub fn iter(&self) -> std::slice::Iter<String> {
        self.ctors.iter()
    }
}

impl Section for CtorsSection {
    const KIND: SectionKind = SectionKind::Ctors;
}
