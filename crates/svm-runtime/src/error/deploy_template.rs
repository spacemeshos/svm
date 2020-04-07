/// Signifies deploy-template failure
#[derive(Debug, PartialEq, Clone)]
pub enum DeployTemplateError {
    /// Out-of-Gas
    OOG,
}

impl ToString for DeployTemplateError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
