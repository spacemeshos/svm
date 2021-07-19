#![allow(unused)]

use clap::ArgMatches;
use thiserror::Error;

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::rc::Rc;
use std::str::Utf8Error;

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
        .subcommand(SubCommand::with_name("tx-spawn").about("Crafts a transaction of type `spawn`"))
        .subcommand(
            SubCommand::with_name("tx-deploy").about("Crafts a transactino of type`deploy`"),
        )
        .subcommand(SubCommand::with_name("tx-call").about("Crafts a transaction of type `call`"))
}

#[derive(Clone, Debug, Error)]
enum Error {
    #[error("Invalid UTF-8 in .wat file.")]
    InvalidUtf8(#[from] Utf8Error),
    #[error("Unknown file extension. Only .wat, .wast and .wasm are supported.")]
    UnknownFileExtension,
}

fn main() -> anyhow::Result<()> {
    let cli_matches = clap_app().get_matches();
    match cli_matches.subcommand() {
        ("validate", Some(args)) => subcmd_validate(args)?,
        ("tx-spawn", Some(args)) => subcmd_validate(args)?,
        ("tx-deploy", Some(args)) => subcmd_validate(args)?,
        ("tx-call", Some(args)) => subcmd_validate(args)?,
        (_, _) => unreachable!(),
    }
    Ok(())
}

fn subcmd_validate(cli_matches: &ArgMatches) -> anyhow::Result<()> {
    let file_path = cli_matches.value_of("input").unwrap();
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
