use crate::{Section, SectionKind};

#[derive(Debug, Clone, PartialEq)]
pub struct ApiSection {
    // TODO: in the future...
}

impl Section for ApiSection {
    const KIND: SectionKind = SectionKind::Api;
}
