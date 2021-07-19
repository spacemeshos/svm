#![allow(unused)]

use clap::ArgMatches;
use thiserror::Error;

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use std::str::Utf8Error;
use std::sync::Arc;

use svm_gas::resolvers::ExampleResolver;
use svm_gas::validate_wasm;
use svm_gas::ProgramPricing;
use svm_program::{Program, ProgramVisitor};

fn clap_app() -> clap::App<'static, 'static> {
    use clap::*;

    // Help messages all use the third person rather than the imperative form,
    // e.g. "prints" rather than "print".

    App::new("svm-cli")
        .version("1.0")
        .author("The Spacemesh team")
        .about("A CLI tool to access SVM internal utilities")
        // The user must provide a valid subcommand, otherwise we don't really
        // know what to do.
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("validate")
                .about("Runs validation logic on a smWasm file")
                .arg(
                    Arg::with_name("input")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("tx")
                .about("Crafts a transaction and writes its byte representation to a file")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .help("Writes the output to a binary file")
                        .takes_value(true),
                )
                .subcommand(
                    SubCommand::with_name("spawn")
                        .about("Crafts a transaction of type `spawn`")
                        .arg(
                            Arg::with_name("version")
                                .default_value("0")
                                .short("V")
                                .long("Version")
                                .help("Template version"),
                        )
                        .arg(
                            Arg::with_name("name")
                                .required(true)
                                .long("name")
                                .help("The name of the `spawn`'ed Account.")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("template")
                                .required(true)
                                .long("template")
                                .takes_value(true)
                                .help("The name of the Account's Template."),
                        )
                        .arg(
                            Arg::with_name("ctor")
                                .long("ctor")
                                .required(true)
                                .takes_value(true)
                                .help("The `ctor` of the Template"),
                        )
                        .arg(
                            Arg::with_name("calldata")
                                .long("calldata")
                                .required(true)
                                .takes_value(true)
                                .help("A binary file containing the calldata"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("deploy").about("Crafts a transactino of type`deploy`"),
                )
                .subcommand(
                    SubCommand::with_name("call").about("Crafts a transaction of type `call`"),
                ),
        )
}

#[derive(Clone, Debug, Error)]
enum Error {
    #[error("Invalid UTF-8 in .wat file.")]
    InvalidUtf8(#[from] Utf8Error),
    #[error("Unknown file extension. Only .wat, .wast and .wasm are supported.")]
    UnknownFileExtension,
}

fn main() -> anyhow::Result<()> {
    let args = clap_app().get_matches();
    match args.subcommand() {
        ("validate", Some(args)) => subcmd_validate(args)?,
        ("tx", Some(args)) => subcmd_tx(args)?,
        (_, _) => unreachable!(),
    }
    Ok(())
}

fn subcmd_tx(args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        ("spawn", Some(spawn_args)) => subcmd_tx_spawn(args, spawn_args)?,
        ("call", Some(args)) => unimplemented!("https://github.com/spacemeshos/svm/issues/304"),
        ("deploy", Some(args)) => unimplemented!("https://github.com/spacemeshos/svm/issues/303"),
        (_, _) => unreachable!(),
    }
    Ok(())
}

fn subcmd_tx_spawn(tx_args: &ArgMatches, spawn_args: &ArgMatches) -> anyhow::Result<()> {
    let mut file = File::create(tx_args.value_of("file").unwrap())?;

    let version = {
        let s = spawn_args.value_of("version").unwrap();
        u16::from_str(s)?
    };
    let ctor = spawn_args.value_of("ctor").unwrap();
    let name = spawn_args.value_of("name").unwrap();
    let name = spawn_args.value_of("calldata").unwrap();
    let template_addr = {
        let s = spawn_args.value_of("template").unwrap();
        svm_types::Address::of(s)
    };
    let builder = svm_codec::api::builder::SpawnBuilder::new()
        .with_ctor(ctor)
        .with_version(version)
        .with_template(&template_addr.into())
        .with_name(name);

    file.write_all(&builder.build())?;
    Ok(())
}

fn subcmd_validate(args: &ArgMatches) -> anyhow::Result<()> {
    let file_path = args.value_of("input").unwrap();
    let file_contents = std::fs::read(file_path)?;

    let program_res = if file_path.ends_with(".wat") || file_path.ends_with(".wast") {
        std::str::from_utf8(&file_contents)
            .map_err(|e| {
                println!("[ERROR] .wat files MUST be valid UTF-8.");
                Error::from(e)
            })
            .map(|s| Program::from_wat(s, false))
    } else if file_path.ends_with(".wasm") {
        Ok(Program::new(&file_contents, false))
    } else {
        Err(Error::UnknownFileExtension)
    }?;

    match program_res {
        Ok(program) => {
            println!("The given file contains a valid smWasm module.");

            let resolver = ExampleResolver::default();
            let mut pp = ProgramPricing::new(Rc::new(resolver));
            let func_price = pp.visit(&program).unwrap();

            println!("{}", func_price);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}
