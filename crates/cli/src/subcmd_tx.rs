use clap::ArgMatches;
use serde_json::{json, Value};

use std::fs::File;
use std::io::{self, Write};
use std::str::FromStr;

use svm_codec::api::json;

pub fn clap_app_tx() -> clap::App<'static, 'static> {
    use clap::*;

    SubCommand::with_name("tx")
        .about("Low-level API to craft transactions from JSON specification files")
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
                .possible_values(&["spawn", "call"]),
        )
}

pub fn subcmd_tx(args: &ArgMatches) -> anyhow::Result<()> {
    let action = match args.value_of("tx-type").unwrap() {
        "spawn" => Action::Spawn,
        "call" => Action::Call,
        _ => unreachable!(),
    };

    let input_path = args.value_of("input").unwrap();
    let input_s = std::fs::read_to_string(input_path)?;
    let bytes = match action {
        Action::Call => json::encode_call_raw(&input_s).expect("Invalid JSON"),
        Action::Spawn => json::encode_spawn(&input_s).expect("Invalid JSON"),
    };

    let mut file = File::create(args.value_of("output").unwrap())?;
    file.write_all(&bytes)?;

    Ok(())
}

fn encode_spawn(input: &str) -> Vec<u8> {
    let mut object = as_json(input);
    assert!(
        matches!(object, Value::Object(..)),
        "Expected a JSON Object"
    );

    let verifydata = encode_input(&object, "verifydata");
    let calldata = encode_input(&object, "calldata");

    let mut map = object.as_object_mut().unwrap();
    map.insert("verifydata".to_string(), verifydata);
    map.insert("calldata".to_string(), calldata);

    json::encode_spawn(&object.to_string()).expect("Invalid JSON")
}

fn encode_input(object: &Value, field: &str) -> Value {
    let input = object.get(field).unwrap();

    if let Value::Object(v) = input {
        assert!(v.contains_key("abi"));
        assert!(v.contains_key("data"));

        let mut encoded = json::encode_inputdata(&input.to_string()).unwrap();
        encoded["data"].take()
    } else {
        unreachable!()
    }
}

fn as_json(s: &str) -> Value {
    serde_json::from_str(s).unwrap()
}

enum Action {
    Spawn,
    Call,
}
