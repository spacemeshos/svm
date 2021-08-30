//!
//! # `Data Section`
//!
//! +------------+----------------+-----------+
//! |            |                |           |
//! |  #Layouts  |  Layout #1     |    ...    |
//! | (2 bytes)  | (see `Layout`) |           |
//! |            |                |           |
//! +------------+----------------+-----------+
//!
//!
//! ## `Layout`
//!
//! +--------------+-----------------------------------------+
//! |              |                                         |
//! | Layout Kind  |        Layout Specific Encoding         |
//! |  (2 bytes)   |          (see `Fixed Layout`)           |
//! |              |                                         |
//! +--------------+-----------------------------------------+
//!
//!
//! ### `Fixed Layout`
//!
//! Right now, there is only the `Fixed Layout`
//!
//! When `#Vars > 0`
//! +--------------+----------------------------+-------------------+-------------------+
//! |              |           |                |                   |                   |
//! |   0x00_01    |   #Vars   |  First Var Id  |  Var #0 Byte-Size |       ...         |
//! |  (2 bytes)   | (4 bytes) |    (4 bytes)   |    (2 bytes)      |                   |
//! |              |           |                |                   |                   |
//! +--------------+-----------+----------------+-------------------+-------------------+
//!
//!
//! When `#Vars = 0`
//! +--------------+-----------+
//! |              |           |
//! |   0x00_01    |     0     |  
//! |  (2 bytes)   | (4 bytes) |
//! |              |           |
//! +--------------+-----------+
//!
//!
//!

use std::convert::TryFrom;

use svm_layout::{FixedLayoutBuilder, Id, Layout, LayoutKind};
use svm_types::DataSection;

use crate::{Codec, ParseError, ReadExt, WriteExt};

pub const FIXED: u16 = 0x00_01;

impl Codec for DataSection {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        u16::try_from(self.len()).unwrap().encode(w);

        for layout in self.layouts() {
            layout.encode(w);
        }
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, ParseError> {
        let layout_count = u16::decode(reader)? as usize;

        // Decoding each `Layout`
        let mut section = DataSection::with_capacity(layout_count);

        for _ in 0..layout_count {
            let layout = Layout::decode(reader)?;

            section.add_layout(layout);
        }

        Ok(section)
    }
}

impl Codec for Layout {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.kind().encode(w);

        match self.kind() {
            LayoutKind::Fixed => {
                let layout = self.as_fixed();

                // `#Vars`
                let var_count = layout.len();
                assert!(var_count < std::u16::MAX as usize);
                (var_count as u16).encode(w);

                if var_count > 0 {
                    // `First Var Id`
                    let first = layout.first();
                    (first.0 as u32).encode(w);

                    // Encoding each `Var Byte-Size`
                    for var in layout.iter() {
                        (var.byte_size() as u16).encode(w);
                    }
                }
            }
        }
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        // `Layout Kind`
        let kind = LayoutKind::decode(reader)?;

        match kind {
            LayoutKind::Fixed => {
                // `#Vars
                let var_count = u16::decode(reader)?;
                let var_count = var_count as usize;

                let mut builder = FixedLayoutBuilder::with_capacity(var_count);

                if var_count > 0 {
                    // `First Var Id`
                    let first = Id(u32::decode(reader)?);
                    builder.set_first(first);

                    // Decoding each `var`
                    for _ in 0..var_count {
                        let byte_size = decode_var_byte_size(reader)?;

                        builder.push(byte_size);
                    }
                }

                let fixed = builder.build();
                let layout = Layout::Fixed(fixed);

                Ok(layout)
            }
        }
    }
}

impl Codec for LayoutKind {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        match self {
            LayoutKind::Fixed => FIXED,
        }
        .encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        match u16::decode(reader)? {
            FIXED => Ok(LayoutKind::Fixed),
            _ => unreachable!(),
        }
    }
}

fn decode_var_byte_size(reader: &mut impl ReadExt) -> Result<u32, ParseError> {
    Ok(u16::decode(reader)? as u32)
}
