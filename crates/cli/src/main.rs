//! A CLI tool that exposes SVM internals and makes it easy to craft
//! transactions by compiling human-readable JSON files down to the SVM-specific
//! ABI.

#![allow(unused)]
#![deny(rustdoc::broken_intra_doc_links)]

mod subcmd_craft_deploy;
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

use subcmd_craft_deploy::{clap_app_craft_deploy, subcmd_craft_deploy};
use subcmd_tx::{clap_app_tx, subcmd_tx};
use subcmd_validate::{clap_app_validate, subcmd_validate};

fn main() -> anyhow::Result<()> {
    let clap_matches = clap_app().get_matches();
    match clap_matches.subcommand() {
        Some(("validate", args)) => subcmd_validate(args)?,
        Some(("tx", args)) => subcmd_tx(args)?,
        Some(("craft-deploy", args)) => subcmd_craft_deploy(args)?,
        _ => unreachable!(),
    }
    Ok(())
}

#[derive(Clone, Debug, Error)]
enum Error {
    #[error("Invalid UTF-8 in .wat file.")]
    InvalidUtf8(#[from] Utf8Error),
    #[error("Unknown file extension. Only .wat, .wast and .wasm are supported.")]
    UnknownFileExtension,
}

fn clap_app() -> clap::App<'static> {
    use clap::*;

    // Help messages all use the third person rather than the imperative form,
    // e.g. "prints" rather than "print".

    App::new("svm-cli")
        .version("1.0")
        .author("The Spacemesh Team")
        .about("A CLI tool to access SVM internal utilities")
        // The user must provide a valid subcommand, otherwise we don't really
        // know what to do.
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(clap_app_validate())
        .subcommand(clap_app_tx())
        .subcommand(clap_app_craft_deploy())
}
