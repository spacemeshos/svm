extern crate clap;

use std::error::Error;
use clap::{value_t, App, AppSettings, Arg, ArgMatches};
use crate::{app_template};

pub fn new_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("SVM CLI")
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
                        )
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
        );

    return app;
}

pub fn process(matches: ArgMatches) -> Result<String, Box<dyn Error>> {
    match matches.subcommand() {
        ("encode", Some(matches)) => match matches.subcommand() {
            ("app-template", Some(matches)) => {
                let version = value_t!(matches, "version", u32).unwrap_or_else(|e| e.exit());
                let name = matches.value_of("name").unwrap();
                let page_count = value_t!(matches, "page_count", u16).unwrap_or_else(|e| e.exit());
                let code_path = matches.value_of("code_path").unwrap();
                let output_path = matches.value_of("output_path").unwrap();

                let num = app_template::encode(version, name, page_count, code_path, output_path)?;
                return Ok(format!("Wrote {} bytes to {}", num, output_path))
            }
            _ => unreachable!(),
        },
        ("decode", Some(matches)) => match matches.subcommand() {
            ("app-template", Some(matches)) => {
                let data_path = matches.value_of("data_path").unwrap();

                let t = app_template::decode(data_path)?;
                return Ok(format!("{:?}", t));
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
