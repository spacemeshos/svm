use serde_json::Value;

use svm_codec::SectionsEncoder;
use svm_layout::{FixedLayoutBuilder, Id, Layout};
use svm_types::{CtorsSection, DataSection, Section, Sections};

pub fn emit_binary_sections(sections: &Sections) -> Vec<u8> {
    let mut encoder = SectionsEncoder::with_capacity(sections.len());
    encoder.encode(sections);

    encoder.finish()
}

pub fn build_sections(meta: &Value) -> Sections {
    let ctors = ctors_section(meta);
    // let data = data_section(meta);

    let mut sections = Sections::with_capacity(2);

    // sections.insert(Section::Ctors(ctors));
    // sections.insert(Section::Data(data));

    sections
}

fn ctors_section(meta: &Value) -> CtorsSection {
    let mut section = CtorsSection::default();

    // for ctor in meta.ctors() {
    // let ctor = ctor.api_name.clone();
    // section.push(ctor);
    // }

    section
}

// fn data_section(meta: &TemplateMeta) -> DataSection {
//     let mut section = DataSection::with_capacity(1);

//     let mut builder = FixedLayoutBuilder::default();
//     builder.set_first(Id(0));

//     for var in meta.storage() {
//         match *var {
//             Var::Primitive { byte_count, .. } => builder.push(byte_count as u32),
//             Var::Array {
//                 byte_count, length, ..
//             } => {
//                 for _ in 0..length {
//                     builder.push(byte_count as u32);
//                 }
//             }
//         }
//     }

//     let layout = builder.build();
//     section.add_layout(Layout::Fixed(layout));

//     section
// }
