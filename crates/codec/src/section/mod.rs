pub mod decode;
pub mod encode;

pub mod preview;
pub mod sections;

pub use decode::SectionsDecoder;
pub use encode::SectionsEncoder;
pub use preview::SectionPreview;

use crate::{Codec, ParseError, ReadExt, WriteExt};
use svm_types::{Sections, Template};

impl Codec for Sections {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        let template = Template::from_sections(self.clone());
        let bytes = crate::template::encode(&template);
        w.write_bytes(&bytes);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let template = crate::template::decode(reader.as_cursor(), None)?;
        Ok(template.sections().clone())
    }
}
