use regex::Regex;

use crate::common::{fmt_addr, fmt_args, fmt_buf};

mod common;
use svm_cli::cli;
use svm_types::Address;

struct AppTxTestCase {
    version: String,
    app_addr_hex: String,
    func_idx: String,
    func_buf: String,
    func_args: Vec<String>,
}

#[test]
fn encode_decode() {
    let cases = vec![
        AppTxTestCase {
            version: String::from("0"),
            app_addr_hex: String::from("00aa00aa00aa00aa00aa00aa00aa00aa00aa00aa"),
            func_idx: String::from("0"),
            func_buf: String::from("11bb11bb12345678"),
            func_args: vec![String::from("10i32"), String::from("20i64")],
        },
        AppTxTestCase {
            version: String::from("0"),
            app_addr_hex: String::from("00aa00aa00aa00aa00aa00aa00aa00aa00aa00aa"),
            func_idx: String::from("12"),
            func_buf: String::from(""),
            func_args: vec![String::from("1073741824i64"), String::from("0i32")],
        },
    ];

    for case in cases {
        test_encode_decode(case)
    }
}

fn test_encode_decode(case: AppTxTestCase) {
    let tempfile_path = tempfile::NamedTempFile::new().unwrap();
    let tempfile_path = tempfile_path.path().to_str().unwrap();

    let output_path = tempfile_path;
    let mut input = vec![
        "myprog",
        "encode",
        "app_tx",
        output_path,
        &case.version,
        &case.app_addr_hex,
        &case.func_idx,
        &case.func_buf,
    ];
    input.extend_from_slice(
        &case
            .func_args
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
    );

    let matches = cli::new_app().get_matches_from(input);
    let output = cli::process(matches).unwrap();

    let re = Regex::new(r"Wrote (\d+) bytes to (.*)").unwrap();
    let cap = re.captures(&output).unwrap();
    assert_eq!(&cap[2], output_path);

    let data_path = output_path;
    let input = vec!["myprog", "decode", "app_tx", data_path];
    let matches = cli::new_app().get_matches_from(input);
    let output = cli::process(matches).unwrap();

    let re =
        Regex::new(r"Version: (.*)\nApp: (.*)\nfunc_idx: (\d+)\nfunc_buf: (.*)\nfunc_args: (.*)")
            .unwrap();
    let caps = re.captures(&output).unwrap();

    assert_eq!(&caps[1], case.version);
    assert_eq!(&caps[2], fmt_addr(&case.app_addr_hex));
    assert_eq!(&caps[3], case.func_idx);
    assert_eq!(&caps[4], fmt_buf(&case.func_buf));
    assert_eq!(&caps[5], fmt_args(case.func_args));
}

#[test]
fn encode_invalid_outputpath() {
    let output_path = "";
    let version = "0";
    let app_addr_hex = "00aa00aa00aa00aa00aa00aa00aa00aa00aa00aa";
    let func_idx = "0";

    let input = vec![
        "myprog",
        "encode",
        "app_tx",
        output_path,
        version,
        app_addr_hex,
        func_idx,
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
fn encode_invalid_app_addr() {
    let output_path = "";
    let version = "0";
    let app_addr_hex = "00aa00aa00aa00aa00aa";
    let func_idx = "0";

    let input = vec![
        "myprog",
        "encode",
        "app_tx",
        output_path,
        version,
        app_addr_hex,
        func_idx,
    ];

    let matches = cli::new_app().get_matches_from(input);
    let res = cli::process(matches);
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        format!(
            "invalid address length: found {}, expected: {}",
            hex::decode(app_addr_hex).unwrap().len(),
            Address::len()
        )
    );
}

#[test]
fn encode_invalid_hex() {
    let invalid_hex_str = "00a";
    assert!(hex::decode(invalid_hex_str).is_err());

    // Invalid `app_addr_hex`.
    let output_path = "";
    let version = "0";
    let app_addr_hex = invalid_hex_str;
    let func_idx = "0";

    let input = vec![
        "myprog",
        "encode",
        "app_tx",
        output_path,
        version,
        app_addr_hex,
        func_idx,
    ];

    let matches = cli::new_app().get_matches_from(input);
    let res = cli::process(matches);
    assert!(res.is_err());
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .starts_with("failed to decode hex string"));

    // Invalid `func_buf`.
    let output_path = "";
    let version = "0";
    let app_addr_hex = "00aa00aa00aa00aa00aa00aa00aa00aa00aa00aa";
    let func_idx = "0";
    let func_buf = invalid_hex_str;

    let input = vec![
        "myprog",
        "encode",
        "app_tx",
        output_path,
        version,
        app_addr_hex,
        func_idx,
        func_buf,
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
