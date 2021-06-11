#[derive(Clone)]
pub struct HeaderSection {
    svm_version: u32,

    executable: bool,

    name: String,

    desc: String,
}

impl HeaderSection {
    pub fn svm_version(&self) -> u32 {
        self.svm_version
    }

    pub fn is_executable(&self) -> bool {
        self.executable
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }
}
