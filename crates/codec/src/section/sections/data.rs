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

use svm_layout::{FixedLayoutBuilder, Id, Layout, LayoutKind, RawVar};
use svm_types::DataSection;

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Codec, Field, ParseError, ReadExt};

pub const FIXED: u16 = 0x00_01;

impl SectionEncoder for DataSection {
    fn encode(&self, w: &mut Vec<u8>) {
        // `#Layouts`
        encode_layout_count(self.len(), w);

        // Encoding each `Layout`
        for layout in self.layouts() {
            encode_layout(layout, w);
        }
    }
}

impl SectionDecoder for DataSection {
    fn decode(cursor: &mut impl ReadExt) -> Result<Self, ParseError> {
        // `#Layouts`
        let layout_count = decode_layout_count(cursor)? as usize;

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
                encode_var_id(first, w);

                // Encoding each `Var Byte-Size`
                for var in layout.iter() {
                    encode_var_byte_size(&var, w);
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
            match u16::decode(cursor) {
                Err(..) => Err(ParseError::Eof(Field::RawVarCount.to_string())),
                Ok(var_count) => {
                    let var_count = var_count as usize;

                    let mut builder = FixedLayoutBuilder::with_capacity(var_count);

                    if var_count > 0 {
                        // `First Var Id`
                        let first = decode_var_id(cursor)?;
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
    }
}

fn encode_layout_kind(kind: LayoutKind, w: &mut Vec<u8>) {
    let raw = match kind {
        LayoutKind::Fixed => FIXED,
    };

    (raw as u16).encode(w);
}

fn decode_layout_kind(cursor: &mut impl ReadExt) -> Result<LayoutKind, ParseError> {
    let value = u16::decode(cursor);

    if value.is_err() {
        return Err(ParseError::Eof(Field::LayoutKind.to_string()));
    }

    match value.unwrap() {
        FIXED => Ok(LayoutKind::Fixed),
        _ => unreachable!(),
    }
}

fn encode_layout_count(layout_count: usize, w: &mut Vec<u8>) {
    assert!(layout_count < u16::MAX as usize);

    (layout_count as u16).encode(w);
}

fn decode_layout_count(cursor: &mut impl ReadExt) -> Result<u16, ParseError> {
    let value = u16::decode(cursor);

    value.map_err(|_| ParseError::Eof(Field::LayoutCount.to_string()))
}

fn encode_var_id(id: Id, w: &mut Vec<u8>) {
    (id.0 as u32).encode(w);
}

fn decode_var_id(cursor: &mut impl ReadExt) -> Result<Id, ParseError> {
    match u32::decode(cursor) {
        Ok(id) => Ok(Id(id)),
        Err(..) => Err(ParseError::Eof(Field::LayoutFirstVarId.to_string())),
    }
}

fn encode_var_byte_size(var: &RawVar, w: &mut Vec<u8>) {
    (var.byte_size() as u16).encode(w);
}

fn decode_var_byte_size(cursor: &mut impl ReadExt) -> Result<u32, ParseError> {
    match u16::decode(cursor) {
        Ok(byte_size) => Ok(byte_size as u32),
        Err(..) => Err(ParseError::Eof(Field::RawVarSize.to_string())),
    }
}
