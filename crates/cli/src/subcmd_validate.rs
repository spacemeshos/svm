use clap::ArgMatches;

use std::io;
use std::rc::Rc;

use svm_gas::{resolvers::ExampleResolver, ProgramPricing};
use svm_program::{Program, ProgramVisitor};

use crate::Error;

pub fn clap_app_validate() -> clap::App<'static, 'static> {
    use clap::*;

    SubCommand::with_name("validate")
        .about("Runs validation logic on a smWasm file")
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
}

pub fn subcmd_validate(args: &ArgMatches) -> anyhow::Result<()> {
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
