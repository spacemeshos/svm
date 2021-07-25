use svm_codec::template;
use svm_layout::{FixedLayoutBuilder, Id, Layout};
use svm_types::{CtorsSection, DataSection, Section, SectionKind};

use crate::program::Program;
use crate::r#struct::Var;

// * Auto-generated `ctors` [`Section`].
// * Auto-generated `data` [`Section`].

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
