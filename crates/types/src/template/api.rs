use crate::{Section, SectionKind};

/// TODO: ...
#[derive(Debug, Clone, PartialEq)]
pub struct ApiSection {
    // TODO: in the future...
}

impl Section for ApiSection {
    const KIND: SectionKind = SectionKind::Api;
}
