use crate::{SectionLike, SectionKind};

/// TODO: ...
/// See <https://github.com/spacemeshos/svm/issues/277>.
#[derive(Debug, Clone, PartialEq)]
pub struct ApiSection {
    // TODO: in the future...
// See <https://github.com/spacemeshos/svm/issues/277>.
}

impl SectionLike for ApiSection {
    const KIND: SectionKind = SectionKind::Api;
}
