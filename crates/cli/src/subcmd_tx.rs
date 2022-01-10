use clap::ArgMatches;
use serde_json::{json, Value};

use std::fs::File;
use std::io::{self, Write};
use std::str::FromStr;

use svm_codec::api::json;

pub fn clap_app_tx() -> clap::App<'static> {
    use clap::*;

    SubCommand::with_name("tx")
        .about("Low-level API to craft transactions from JSON specification files")
        .arg(
            Arg::with_name("input")
                .help("Reads JSON-formatted transactions from this file")
                .short('i')
                .long("input")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Writes the binary output to this file")
                .short('o')
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
    let input_path = args.value_of("input").unwrap();
    let input = std::fs::read_to_string(input_path)?;

    let bytes = match args.value_of("tx-type").unwrap() {
        "spawn" => encode_spawn(&input),
        "call" => encode_call(&input),
        _ => unreachable!(),
    };

    let mut file = File::create(args.value_of("output").unwrap())?;
    file.write_all(&bytes)?;

    Ok(())
}

fn encode_spawn(object_str: &str) -> Vec<u8> {
    let object = encode_inputs(object_str, &["calldata"]);
    json::encode_spawn(&object.to_string()).expect("Invalid JSON")
}

fn encode_call(object_str: &str) -> Vec<u8> {
    let object = encode_inputs(object_str, &["verifydata", "calldata"]);
    json::encode_call_raw(&object.to_string()).expect("Invalid JSON")
}

fn encode_inputs(object_str: &str, keys: &[&str]) -> Value {
    let mut object = serde_json::from_str(object_str).unwrap();
    assert!(
        matches!(object, Value::Object(..)),
        "Expected a JSON Object"
    );

    for key in keys {
        update_key(&mut object, key, encode_input);
    }
    object
}

fn update_key<F>(object: &mut Value, key: &str, f: F)
where
    F: Fn(&Value, &str) -> Value,
{
    let new_value = f(&object, key);
    let mut map = object.as_object_mut().unwrap();
    map.insert(key.to_string(), new_value);
}

fn encode_input(object: &Value, key: &str) -> Value {
    let input = object.get(key).unwrap();

    if let Value::Object(v) = input {
        assert!(v.contains_key("abi"));
        assert!(v.contains_key("data"));

        let mut data = json::encode_inputdata(&input.to_string()).unwrap();
        data["data"].take()
    } else {
        panic!("Expected `abi and `data` under root object key {}", key)
    }
}
