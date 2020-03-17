extern crate svm_cli;

use svm_cli::cli;

fn main() {
    let matches = cli::new_app().get_matches();
    match cli::process(matches) {
        Ok(v) => println!("{}", v),
        Err(e) => eprintln!("{}", e),
    }
}
