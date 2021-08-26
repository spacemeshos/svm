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

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Codec, ParseError, ReadExt};

pub const FIXED: u16 = 0x00_01;

impl SectionEncoder for DataSection {
    fn encode(&self, w: &mut Vec<u8>) {
        u16::try_from(self.len()).unwrap().encode(w);

        for layout in self.layouts() {
            encode_layout(layout, w);
        }
    }
}

impl SectionDecoder for DataSection {
    fn decode(cursor: &mut impl ReadExt) -> Result<Self, ParseError> {
        let layout_count = u16::decode(cursor)? as usize;

        // Decoding each `Layout`
        let mut section = DataSection::with_capacity(layout_count);

        for _ in 0..layout_count {
            let layout = decode_layout(cursor)?;

            section.add_layout(layout);
        }

        Ok(section)
    }
}

fn encode_layout(layout: &Layout, w: &mut Vec<u8>) {
    // `Layout Kind`
    let kind = layout.kind();

    encode_layout_kind(kind, w);

    match kind {
        LayoutKind::Fixed => {
            let layout = layout.as_fixed();

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

fn decode_layout(cursor: &mut impl ReadExt) -> Result<Layout, ParseError> {
    // `Layout Kind`
    let kind = decode_layout_kind(cursor)?;

    match kind {
        LayoutKind::Fixed => {
            // `#Vars
            let var_count = u16::decode(cursor)?;
            let var_count = var_count as usize;

            let mut builder = FixedLayoutBuilder::with_capacity(var_count);

            if var_count > 0 {
                // `First Var Id`
                let first = Id(u32::decode(cursor)?);
                builder.set_first(first);

                // Decoding each `var`
                for _ in 0..var_count {
                    let byte_size = decode_var_byte_size(cursor)?;

                    builder.push(byte_size);
                }
            }

            let fixed = builder.build();
            let layout = Layout::Fixed(fixed);

            Ok(layout)
        }
    }
}

fn encode_layout_kind(kind: LayoutKind, w: &mut Vec<u8>) {
    let raw = match kind {
        LayoutKind::Fixed => FIXED,
    };

    (raw as u16).encode(w);
}

fn decode_layout_kind(cursor: &mut impl ReadExt) -> Result<LayoutKind, ParseError> {
    let value = u16::decode(cursor)?;

    match value {
        FIXED => Ok(LayoutKind::Fixed),
        _ => unreachable!(),
    }
}

fn decode_var_byte_size(cursor: &mut impl ReadExt) -> Result<u32, ParseError> {
    Ok(u16::decode(cursor)? as u32)
}
