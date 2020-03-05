use crate::types::AppTemplate;

use svm_common::Address;

#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub struct DeployAppTemplate {
    pub template: AppTemplate,

    pub author: Address,
}
