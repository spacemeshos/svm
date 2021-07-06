//!  `Template` Binary Format
//!
//!  Important: There are no assumptions regarding Sections order
//!
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
//!  | Deploy Section | (Optional, will be derived from the `Transaction Envelope`)
//!  |                |
//!  +----------------+
//!
//!

use std::collections::HashSet;
use std::io::Cursor;

use svm_types::{SectionKind, Template};

use crate::section::decode::decode_sections;
use crate::section::SectionsEncoder;
use crate::ParseError;

pub fn encode(template: &Template) -> Vec<u8> {
    let sections = template.sections();

    let mut encoder = SectionsEncoder::with_capacity(sections.len());
    encoder.encode(sections);

    let bytes = encoder.finish();

    bytes
}

pub fn decode(
    cursor: Cursor<&[u8]>,
    interests: Option<HashSet<SectionKind>>,
) -> Result<Template, ParseError> {
    let sections = decode_sections(cursor, interests)?;

    let template = Template::new(sections);
    Ok(template)
}

#[cfg(test)]
mod tests {
    use super::*;

    use maplit::hashset;

    use svm_layout::{FixedLayout, Id, Layout, RawVar};
    use svm_types::{
        Address, CodeKind, CodeSection, CtorsSection, DataSection, DeploySection, GasMode,
        HeaderSection, Layer, Nonce, Sections, TemplateAddr, TransactionId,
    };

    fn make_code_section() -> CodeSection {
        CodeSection::new(CodeKind::Wasm, vec![0xC0, 0xDE], 0x01, GasMode::Fixed, 1)
    }

    fn make_data_section() -> DataSection {
        let mut section = DataSection::default();

        let var1 = RawVar::new(Id(0), 0, 10);
        let var2 = RawVar::new(Id(1), 10, 5);

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
        let nonce = Nonce(20);
        let deployer = Address::repeat(0xAB).into();
        let template = Address::repeat(0xCD).into();

        DeploySection::new(tx_id, layer, nonce, deployer, template)
    }

    #[test]
    fn encode_template() {
        let code = make_code_section();
        let data = make_data_section();
        let ctors = make_ctors_section();
        let header = make_header_section();

        let mut template = TemplateBuilder::default()
            .with_code(code)
            .with_data(data)
            .with_ctors(ctors)
            .with_header(header)
            .build();

        let deploy = make_deploy_section();
        template.set_deploy_section(deploy);

        let bytes = encode(&template);

        let mut cursor = Cursor::new(&bytes[..]);

        let interests = hashset! {
            SectionKind::Code,
            SectionKind::Data,
            SectionKind::Ctors,
            SectionKind::Header,
            SectionKind::Deploy
        };

        let sections = decode_sections(cursor, Some(interests)).unwrap();

        assert_eq!(template.sections(), &sections);
    }
}
