mod meta;

use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;

use svm_codec::{SectionsDecoder, SectionsEncoder};
use svm_layout::{FixedLayout, Id, Layout};
use svm_types::{CodeSection, CtorsSection, DataSection, Section, SectionKind, Sections};

use meta::TemplateMeta;

pub fn clap_app_craft_deploy() -> clap::App<'static> {
    use clap::*;

    SubCommand::with_name("craft-deploy")
        .about("High-level API to craft \"Deploy\" transactions")
        .arg(
            Arg::with_name("smwasm")
                .help("Path to the smWasm `#[template]` code")
                .long("smwasm")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("meta")
                .help("Path to the JSON meta-information produced by the SVM SDK")
                .long("meta")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Writes the binary output to this file")
                .short('o')
                .long("output")
                .required(true)
                .takes_value(true),
        )
}

pub fn subcmd_craft_deploy(args: &ArgMatches) -> anyhow::Result<()> {
    let smwasm = {
        let path = args.value_of("smwasm").unwrap();
        std::fs::read(path)?
    };
    let meta: TemplateMeta = {
        let path = args.value_of("meta").unwrap();
        let string = std::fs::read_to_string(path)?;
        serde_json::from_str(string.as_str())?
    };

    let flags = CodeSection::exec_flags();
    let code_section = CodeSection::new(
        svm_types::CodeKind::Wasm,
        smwasm,
        flags,
        svm_types::GasMode::Fixed,
        0,
    );

    let mut sections = Sections::with_capacity(3);
    sections.insert(Section::Code(code_section));
    sections.insert(Section::Ctors(meta.ctors_section()));
    sections.insert(Section::Data(meta.data_section()));

    let mut encoder = SectionsEncoder::with_capacity(3);
    encoder.encode(&sections);
    let bytes = encoder.finish();

    let path = args.value_of("output").unwrap();
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;

    test_sections_encoding(&path);
    test_decode_as_template(&path);

    Ok(())
}

fn test_sections_encoding<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let bytes = std::fs::read(&path)?;
    let mut cursor = Cursor::new(bytes.as_slice());
    let mut decoder = SectionsDecoder::new(&mut cursor)?;

    assert_eq!(3, decoder.section_count());

    assert_section(&mut decoder, SectionKind::Code);
    assert_section(&mut decoder, SectionKind::Ctors);
    assert_section(&mut decoder, SectionKind::Data);

    Ok(())
}

fn assert_section(
    decoder: &mut SectionsDecoder<Cursor<&[u8]>>,
    expected: SectionKind,
) -> anyhow::Result<()> {
    let preview = decoder.next_preview()?;
    assert_eq!(preview.kind(), expected);

    let section = decoder.skip_section()?;
    assert_eq!(preview.kind(), expected);

    Ok(())
}

fn test_decode_as_template<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let message = std::fs::read(&path)?;
    let mut cursor = Cursor::new(message.as_slice());
    let _template = svm_codec::template::decode(&mut cursor, None)?;

    Ok(())
}
