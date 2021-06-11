#[derive(Debug, Clone)]
pub struct CtorsSection {
    ctors: Vec<String>,
}

impl CtorsSection {
    pub fn push_ctor(&mut self, ctor: String) {
        self.ctors.push(ctor);
    }

    pub fn ctors(&self) -> &[String] {
        &self.ctors
    }

    pub fn to_vec(self) -> Vec<String> {
        self.ctors
    }
}
