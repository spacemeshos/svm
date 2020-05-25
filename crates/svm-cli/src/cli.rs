use std::error::Error;

use crate::{app_template, app_tx, spawn_app};
use clap::{value_t, App, AppSettings, Arg, ArgMatches};

pub fn new_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("SVM CLI")
        .version("1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("encode")
                .about("Encode various tx/msg types")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("app_template")
                        .about("the `app_template` type")
                        .arg(
                            Arg::with_name("version")
                                .help("the template version")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("name")
                                .help("the template name")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("code_path")
                                .help("path to the file containing the template code data")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("output_path")
                                .help("path to the file to save the encoded raw data"),
                        ),
                )
                .subcommand(
                    App::new("spawn_app")
                        .about("the `spawn_app` type")
                        .arg(
                            Arg::with_name("output_path")
                                .help("path to the file to save the encoded raw data")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("version")
                                .help("the app version")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("template_addr")
                                .help("the app template address, in hex")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("ctor_idx")
                                .help("the app constructor func index")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("ctor_buf")
                                .help("the app constructor buffer data (blob), in hex")
                                .required(false),
                        )
                        .arg(
                            Arg::with_name("ctor_args")
                                .help("the app constructor func arguments")
                                .required(false)
                                .multiple(true),
                        ),
                )
                .subcommand(
                    App::new("app_tx")
                        .about("the `app_tx` type")
                        .arg(
                            Arg::with_name("output_path")
                                .help("path to the file to save the encoded raw data")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("version")
                                .help("the app version")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("app_addr")
                                .help("the app address, in hex")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("func_idx")
                                .help("the func index")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("func_buf")
                                .help("the func buffer data (blob), in hex")
                                .required(false),
                        )
                        .arg(
                            Arg::with_name("func_args")
                                .help("the func arguments")
                                .required(false)
                                .multiple(true),
                        ),
                ),
        )
        .subcommand(
            App::new("decode")
                .about("Decode various tx/msg types")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("app_template")
                        .about("the `app_template` type")
                        .arg(
                            Arg::with_name("data_path")
                                .help("path to the file containing the encoded raw data")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("spawn_app").about("the `spawn_app` type").arg(
                        Arg::with_name("data_path")
                            .help("path to the file containing the encoded raw data")
                            .required(true),
                    ),
                )
                .subcommand(
                    App::new("app_tx").about("the `app_tx` type").arg(
                        Arg::with_name("data_path")
                            .help("path to the file containing the encoded raw data")
                            .required(true),
                    ),
                ),
        );

    return app;
}

pub fn process(matches: ArgMatches) -> Result<String, Box<dyn Error>> {
    match matches.subcommand() {
        ("encode", Some(matches)) => match matches.subcommand() {
            ("app_template", Some(matches)) => {
                let version = value_t!(matches, "version", u32).unwrap_or_else(|e| e.exit());
                let name = matches.value_of("name").unwrap();
                let code_path = matches.value_of("code_path").unwrap();
                let output_path = matches.value_of("output_path").unwrap();

                let num = app_template::encode(version, name, code_path, output_path)?;
                return Ok(format!("Wrote {} bytes to {}", num, output_path));
            }
            ("spawn_app", Some(matches)) => {
                let version = value_t!(matches, "version", u32).unwrap_or_else(|e| e.exit());
                let template_addr_hex = matches.value_of("template_addr").unwrap();
                let ctor_idx = value_t!(matches, "ctor_idx", u16).unwrap_or_else(|e| e.exit());
                let ctor_buf_hex = matches.value_of("ctor_buf");
                let ctor_args: Option<Vec<_>> = matches.values_of("ctor_args").map(|v| v.collect());
                let output_path = matches.value_of("output_path").unwrap();

                let num = spawn_app::encode(
                    version,
                    template_addr_hex,
                    ctor_idx,
                    ctor_buf_hex,
                    ctor_args,
                    output_path,
                )?;
                return Ok(format!("Wrote {} bytes to {}", num, output_path));
            }
            ("app_tx", Some(matches)) => {
                let version = value_t!(matches, "version", u32).unwrap_or_else(|e| e.exit());
                let app_addr_hex = matches.value_of("app_addr").unwrap();
                let func_idx = value_t!(matches, "func_idx", u16).unwrap_or_else(|e| e.exit());
                let func_buf_hex = matches.value_of("func_buf");
                let func_args: Option<Vec<_>> = matches.values_of("func_args").map(|v| v.collect());
                let output_path = matches.value_of("output_path").unwrap();

                let num = app_tx::encode(
                    version,
                    app_addr_hex,
                    func_idx,
                    func_buf_hex,
                    func_args,
                    output_path,
                )?;
                return Ok(format!("Wrote {} bytes to {}", num, output_path));
            }
            _ => unreachable!(),
        },
        ("decode", Some(matches)) => match matches.subcommand() {
            ("app_template", Some(matches)) => {
                let data_path = matches.value_of("data_path").unwrap();

                let t = app_template::decode(data_path)?;
                return Ok(format!("{:?}", t));
            }
            ("spawn_app", Some(matches)) => {
                let data_path = matches.value_of("data_path").unwrap();

                let t = spawn_app::decode(data_path)?;
                return Ok(format!("{:?}", t));
            }
            ("app_tx", Some(matches)) => {
                let data_path = matches.value_of("data_path").unwrap();

                let t = app_tx::decode(data_path)?;
                return Ok(format!("{:?}", t));
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
