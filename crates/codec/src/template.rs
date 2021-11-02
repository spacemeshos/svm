//! Encoding for binary [`Template`]s.
//!
//!  [`Template`] Binary Format
//!
//!  Important: There are no assumptions regarding the order of the `Section`s
//!
//!
//! ```text
//!
//!  +----------------+
//!  |                |
//!  |   #Sections    |
//!  |                |
//!  +----------------+
//!  |                |
//!  |  Code Section  |
//!  |                |
//!  +----------------+
//!  |                |
//!  | Data Section   |
//!  |                |
//!  +----------------+
//!  |                |
//!  | Ctors Section  |
//!  |                |
//!  +----------------+
//!  |                |
//!  | Header Section | (Optional)
//!  |                |
//!  +----------------+
//!  |                |
//!  | Schema Section | (Optional)
//!  |                |
//!  +----------------+
//!  |                |
//!  |  API Section   | (Optional)
//!  |                |
//!  +----------------+
//!  |                |
//!  | Deploy Section | (Optional, will be derived from the `Transaction Envelope` and `Transaction Context`)
//!  |                |
//!  +----------------+
//!
//! ```

use std::collections::HashSet;

use svm_types::{SectionKind, Template};

use crate::section::decode::decode_sections;
use crate::section::SectionsEncoder;
use crate::{ParseError, ReadExt};

/// Encodes a `Template` into binary
///
/// This is essentially equivalent to encoding the `Sections` residing within the `Template`
pub fn encode(template: &Template) -> Vec<u8> {
    let sections = template.sections();

    let mut encoder = SectionsEncoder::with_capacity(sections.len());
    encoder.encode(&sections);

    let bytes = encoder.finish();

    bytes
}

/// Decodes a list of `Section`s that we're interested at (see `interest`
/// parameter) and returns them wrapped within a `Template`.
///
/// If the input `interests` is `None` - decodes any kind `Section` belonging to
/// the `Template` pointed by the input `cursor`.
pub fn decode(
    reader: &mut impl ReadExt,
    interests: Option<HashSet<SectionKind>>,
) -> Result<Template, ParseError> {
    let sections = decode_sections(reader, interests)?;

    if !sections.contains(SectionKind::Code)
        || !sections.contains(SectionKind::Data)
        || !sections.contains(SectionKind::Ctors)
    {
        Err(ParseError::InvalidSection)
    } else {
        let template = Template::from_sections(sections);
        Ok(template)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use maplit::hashset;

    use svm_types::BytesPrimitive;

    use super::*;

    use svm_layout::{FixedLayout, Layout, RawVar};
    use svm_types::{
        Address, CodeKind, CodeSection, CtorsSection, DataSection, DeploySection, GasMode,
        HeaderSection, Layer, TemplateAddr, TransactionId,
    };

    fn make_code_section() -> CodeSection {
        CodeSection::new(CodeKind::Wasm, vec![0xC0, 0xDE], 0x01, GasMode::Fixed, 1)
    }

    fn make_data_section() -> DataSection {
        let mut section = DataSection::default();

        let var1 = RawVar::new(0, 0, 10);
        let var2 = RawVar::new(1, 10, 5);

        let fixed = FixedLayout::new(vec![var1, var2]);
        section.add_layout(Layout::Fixed(fixed));

        section
    }

    fn make_ctors_section() -> CtorsSection {
        let mut section = CtorsSection::default();

        section.push("init".to_string());

        section
    }

    fn make_header_section() -> HeaderSection {
        HeaderSection::new(2, "My Template".to_string(), "A few words".to_string())
    }

    fn make_deploy_section() -> DeploySection {
        let tx_id = TransactionId::repeat(0xFF);
        let layer = Layer(10);
        let deployer = Address::repeat(0xAB);
        let template = TemplateAddr::repeat(0xCD);

        DeploySection::new(tx_id, layer, deployer, template)
    }

    #[test]
    fn encode_template() {
        let code = make_code_section();
        let data = make_data_section();
        let ctors = make_ctors_section();
        let header = make_header_section();

        let mut template = Template::new(code, data, ctors).with_header(Some(header));

        let deploy = make_deploy_section();
        template = template.with_deploy(Some(deploy));

        let bytes = encode(&template);
        let mut cursor = Cursor::new(&bytes[..]);

        let interests = hashset! {
            SectionKind::Code,
            SectionKind::Data,
            SectionKind::Ctors,
            SectionKind::Header,
            SectionKind::Deploy
        };

        let sections = decode_sections(&mut cursor, Some(interests)).unwrap();

        assert_eq!(template.sections(), &sections);
    }
}
