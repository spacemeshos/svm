use regex::Regex;

use svm_cli::{cli, wasm_value};
use svm_common::Address;

struct SpawnAppTestCase {
    version: String,
    template_addr: String,
    ctor_index: String,
    ctor_buf: String,
    ctor_args: Vec<String>,
}

/// TODO:
/// write negative tests
/// write test for parse_str
///

#[test]
fn encode_decode() {
    let cases = vec![
        SpawnAppTestCase {
            version: String::from("0"),
            template_addr: String::from("00aa00aa00aa00aa00aa00aa00aa00aa00aa00aa"),
            ctor_index: String::from("0"),
            ctor_buf: String::from("11bb11bb12345678"),
            ctor_args: vec![String::from("10i32"), String::from("20i64")],
        },
        SpawnAppTestCase {
            version: String::from("0"),
            template_addr: String::from("00aa00aa00aa00aa00aa00aa00aa00aa00aa00aa"),
            ctor_index: String::from("12"),
            ctor_buf: String::from(""),
            ctor_args: vec![String::from("1073741824i64"), String::from("0i32")],
        },
    ];

    for case in cases {
        test_encode_decode(case)
    }
}

fn test_encode_decode(case: SpawnAppTestCase) {
    let tempfile_path = tempfile::NamedTempFile::new().unwrap();
    let tempfile_path = tempfile_path.path().to_str().unwrap();

    let output_path = tempfile_path;
    let mut input = vec![
        "myprog",
        "encode",
        "spawn_app",
        output_path,
        &case.version,
        &case.template_addr,
        &case.ctor_index,
        &case.ctor_buf,
    ];
    for i in 0..case.ctor_args.len() {
        input.push(&case.ctor_args[i]);
    }
    let matches = cli::new_app().get_matches_from(input);
    let output = cli::process(matches).unwrap();

    let re = Regex::new(r"Wrote (\d+) bytes to (.*)").unwrap();
    let cap = re.captures(&output).unwrap();
    assert_eq!(&cap[2], output_path);

    let data_path = output_path;
    let input = vec!["myprog", "decode", "spawn_app", data_path];
    let matches = cli::new_app().get_matches_from(input);
    let output = cli::process(matches).unwrap();

    let re = Regex::new(
        r"Version: (.*)\nTemplate: (.*)\nctor_idx: (\d+)\nctor_buf: (.*)\nctor_args: (.*)",
    )
    .unwrap();
    let caps = re.captures(&output).unwrap();

    assert_eq!(&caps[1], case.version);
    assert_eq!(&caps[2], {
        let template_addr = hex::decode(case.template_addr).unwrap();
        let template_addr = Address::from(template_addr.as_slice());
        template_addr.fmt(4, 4, " ")
    });
    assert_eq!(&caps[3], case.ctor_index);
    assert_eq!(
        &caps[4],
        format!(
            "{:?}",
            hex::decode(case.ctor_buf)
                .unwrap()
                .iter()
                .take(4)
                .collect::<Vec<_>>()
        )
    );
    assert_eq!(
        &caps[5],
        format!("{:?}", {
            case.ctor_args
                .iter()
                .map(|v| wasm_value::parse_str(v).unwrap())
                .collect::<Vec<_>>()
        })
    );
}

#[test]
fn encode_invalid_outputpath() {
    let output_path = "";
    let version = "0";
    let template_addr = "00aa00aa00aa00aa00aa00aa00aa00aa00aa00aa";
    let ctor_index = "0";

    let input = vec![
        "myprog",
        "encode",
        "spawn_app",
        output_path,
        version,
        template_addr,
        ctor_index,
    ];

    let matches = cli::new_app().get_matches_from(input);
    let res = cli::process(matches);
    assert!(res.is_err());
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .starts_with(&format!("failed to create file at {}", output_path)));
}

#[test]
fn encode_invalid_template_addr() {
    let output_path = "";
    let version = "0";
    let template_addr = "00aa00aa00aa00aa00aa";
    let ctor_index = "0";

    let input = vec![
        "myprog",
        "encode",
        "spawn_app",
        output_path,
        version,
        template_addr,
        ctor_index,
    ];

    let matches = cli::new_app().get_matches_from(input);
    let res = cli::process(matches);
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        format!(
            "invalid template address length: found {}, expected: {}",
            hex::decode(template_addr).unwrap().len(),
            Address::len()
        )
    );
}

#[test]
fn encode_invalid_hex() {
    let invalid_hex_str = "00a";
    assert!(hex::decode(invalid_hex_str).is_err());

    // Invalid `template_addr`.
    let output_path = "";
    let version = "0";
    let template_addr = invalid_hex_str;
    let ctor_index = "0";

    let input = vec![
        "myprog",
        "encode",
        "spawn_app",
        output_path,
        version,
        template_addr,
        ctor_index,
    ];

    let matches = cli::new_app().get_matches_from(input);
    let res = cli::process(matches);
    assert!(res.is_err());
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .starts_with("failed to decode hex string"));

    // Invalid `ctor_buf`.
    let output_path = "";
    let version = "0";
    let template_addr = "00aa00aa00aa00aa00aa00aa00aa00aa00aa00aa";
    let ctor_index = "0";
    let ctor_buf = invalid_hex_str;

    let input = vec![
        "myprog",
        "encode",
        "spawn_app",
        output_path,
        version,
        template_addr,
        ctor_index,
        ctor_buf,
    ];

    let matches = cli::new_app().get_matches_from(input);
    let res = cli::process(matches);
    assert!(res.is_err());
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .starts_with("failed to decode hex string"));
}
