use clap::ArgMatches;

use std::fs::File;
use std::io::{self, Write};
use std::str::FromStr;

use svm_codec::api::json;

pub fn clap_app_tx() -> clap::App<'static, 'static> {
    use clap::*;

    SubCommand::with_name("tx")
        .about("Crafts a transaction and writes its byte representation to a file")
        .arg(
            Arg::with_name("input")
                .help("Reads JSON-formatted transactions from this file")
                .short("i")
                .long("input")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Writes the binary output to this file")
                .short("o")
                .long("output")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tx-type")
                .help("The type of input transaction")
                .long("tx-type")
                .required(true)
                .takes_value(true)
                .possible_values(&["spawn", "deploy", "call"]),
        )
}

pub fn subcmd_tx(args: &ArgMatches) -> anyhow::Result<()> {
    let action = match args.value_of("tx-type").unwrap() {
        "spawn" => Action::Spawn,
        "call" => Action::Call,
        "deploy" => Action::Deploy,
        _ => unreachable!(),
    };

    let input_path = args.value_of("input").unwrap();
    let input_s = std::fs::read_to_string(input_path)?;
    let json_value: serde_json::Value = serde_json::from_str(input_s.as_str())?;
    let bytes = match action {
        Action::Call => json::encode_call(&json_value).expect("Invalid JSON"),
        Action::Deploy => json::deploy_template(&json_value).expect("Invalid JSON"),
        Action::Spawn => json::encode_spawn(&json_value).expect("Invalid JSON"),
    };

    let mut file = File::create(args.value_of("output").unwrap())?;
    file.write_all(&bytes)?;

    Ok(())
}

enum Action {
    Spawn,
    Deploy,
    Call,
}
