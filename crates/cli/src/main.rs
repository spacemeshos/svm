#![allow(unused)]

mod clap_app;
mod sections;
mod subcmd_auto_deploy;
mod subcmd_tx;
mod subcmd_validate;

use clap::ArgMatches;
use thiserror::Error;

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use std::str::Utf8Error;
use std::sync::Arc;

use svm_gas::resolvers::ExampleResolver;
use svm_gas::validate_wasm;
use svm_gas::ProgramPricing;
use svm_program::{Program, ProgramVisitor};

use clap_app::clap_app;
use subcmd_auto_deploy::subcmd_auto_deploy;
use subcmd_tx::subcmd_tx;
use subcmd_validate::subcmd_validate;

#[derive(Clone, Debug, Error)]
enum Error {
    #[error("Invalid UTF-8 in .wat file.")]
    InvalidUtf8(#[from] Utf8Error),
    #[error("Unknown file extension. Only .wat, .wast and .wasm are supported.")]
    UnknownFileExtension,
}

fn main() -> anyhow::Result<()> {
    let clap_matches = clap_app().get_matches();
    match clap_matches.subcommand() {
        ("validate", Some(args)) => subcmd_validate(args)?,
        ("tx", Some(args)) => subcmd_tx(args)?,
        ("auto-deploy", Some(args)) => subcmd_auto_deploy(args)?,
        (_, _) => unreachable!(),
    }
    Ok(())
}
