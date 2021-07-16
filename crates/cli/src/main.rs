#![allow(unused)]

use structopt::StructOpt;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;

use svm_gas::resolvers::ExampleResolver;
use svm_gas::validate_wasm;
use svm_gas::ProgramPricing;
use svm_program::{Program, ProgramVisitor};

#[derive(StructOpt, Debug)]
#[structopt(name = "svm")]
struct CLI {
    #[structopt(long)]
    wasm_file: Option<PathBuf>,
}

fn main() {
    let opts = CLI::from_args();

    if let Some(wasm_file) = opts.wasm_file {
        let file = File::open(&wasm_file);

        if file.is_err() {
            println!("File {:?} doesn't exist", &wasm_file);
            return;
        }

        let mut file = file.unwrap();

        let mut wasm = Vec::new();
        let _ = file.read_to_end(&mut wasm).unwrap();

        let program = match Program::new(&wasm, false) {
            Ok(p) => {
                println!("File is a valid restricted-Wasm file (Fixed-Gas pricing can be used!)");
                p
            }
            Err(e) => {
                println!("File is NOT a valid restricted Wasm file: {}", e);
                std::process::exit(1);
            }
        };

        let resolver = ExampleResolver::default();

        let mut pp = ProgramPricing::new(Rc::new(resolver));
        let func_price = pp.visit(&program).unwrap();

        println!("{}", func_price);
    }
}
