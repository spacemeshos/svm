use svm_codec::SectionsEncoder;
use svm_layout::{FixedLayoutBuilder, Id, Layout};
use svm_types::{CtorsSection, DataSection, Section, SectionKind, SectionLike, Sections};

use crate::program::Program;
use crate::r#struct::Var;

pub fn emit_binary_sections(sections: &Sections) -> Vec<u8> {
    let mut encoder = SectionsEncoder::with_capacity(sections.len());
    encoder.encode(sections);

    encoder.finish()
}

pub fn build_sections(program: &Program) -> Sections {
    let ctors = ctors_section(program);
    let data = data_section(program);

    let mut sections = Sections::with_capacity(2);

    sections.insert(Section::Ctors(ctors));
    sections.insert(Section::Data(data));

    sections
}

fn ctors_section(program: &Program) -> CtorsSection {
    let mut section = CtorsSection::default();

    for ctor in program.ctors() {
        let ctor = ctor.api_name.clone();
        section.push(ctor);
    }

    section
}

fn data_section(program: &Program) -> DataSection {
    let mut section = DataSection::with_capacity(1);

    let mut builder = FixedLayoutBuilder::default();
    builder.set_first(Id(0));

    for var in program.storage() {
        match *var {
            Var::Primitive { byte_count, .. } => builder.push(byte_count as u32),
            Var::Array {
                id,
                byte_count,
                length,
                ..
            } => {
                for _ in 0..length {
                    builder.push(byte_count as u32);
                }
            }
        }
    }

    let layout = builder.build();
    section.add_layout(Layout::Fixed(layout));

    section
}
