extern crate clap;
extern crate svm_cli;

use clap::{value_t, App, AppSettings, Arg};
use svm_cli::app_template;

fn main() {
    // Define CLI App.
    let matches = App::new("SVM CLI")
        .version("1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("encode")
                .about("Encode various transaction types")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("app-template")
                        .about("the app-template transaction type")
                        .arg(
                            Arg::with_name("version")
                                .help("the app-template version")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("name")
                                .help("the app-template name")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("page_count")
                                .help("the app-template page count")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("code_path")
                                .help("path to the file containing the template code data")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("output_path")
                                .help("path to the file to save the tx encoded raw data")
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            App::new("decode")
                .about("Decode various transaction types")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("app-template")
                        .about("the template transaction type")
                        .arg(
                            Arg::with_name("data_path")
                                .help("path to the file containing the tx encoded raw data")
                                .required(true),
                        ),
                ),
        )
        .get_matches();

    // Process the user input.
    match matches.subcommand() {
        ("encode", Some(matches)) => match matches.subcommand() {
            ("app-template", Some(matches)) => {
                let version = value_t!(matches, "version", u32).unwrap_or_else(|e| e.exit());
                let name = matches.value_of("name").unwrap();
                let page_count = value_t!(matches, "page_count", u16).unwrap_or_else(|e| e.exit());
                let code_path = matches.value_of("code_path").unwrap();
                let output_path = matches.value_of("output_path").unwrap();

                let num = app_template::encode(version, name, page_count, code_path, output_path)
                    .unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    });
                println!("Wrote {} bytes to {}", num, output_path)
            }
            _ => unreachable!(),
        },
        ("decode", Some(matches)) => match matches.subcommand() {
            ("app-template", Some(matches)) => {
                let data_path = matches.value_of("data_path").unwrap();

                let template = app_template::decode(data_path).unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    std::process::exit(1);
                });
                println!("AppTemplate:");
                println!("\tName: {}", template.name);
                println!("\tPage Count: {}", template.page_count);
                println!("\tCode: {:?}", template.code);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
