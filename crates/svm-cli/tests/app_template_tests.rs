extern crate regex;
extern crate tempfile;

use regex::Regex;
use std::fs::File;
use std::io::Read;
use svm_cli::cli;

struct AppTemplateTestCase {
    name: String,
    page_count: String,
}

#[test]
fn encode_decode() {
    let cases = vec![
        AppTemplateTestCase {
            name: String::from("my name"),
            page_count: String::from("0"),
        },
        AppTemplateTestCase {
            name: String::from("שלום"),
            page_count: String::from("1000"),
        },
        // FAILING:
        AppTemplateTestCase {
            name: String::from("नमस्ते"),
            page_count: String::from("1000"),
        },
    ];

    for case in cases {
        test_encode_decode(case)
    }
}

#[test]
fn encode_invalid_codepath() {
    let version = "0";
    let name = "";
    let page_count = "1";
    let code_path = "invalid_codepath";
    let output_path = "";
    let input = vec![
        "myprog",
        "encode",
        "app-template",
        version,
        name,
        page_count,
        code_path,
        output_path,
    ];
    println!("input: {:?}", input);

    let matches = cli::new_app().get_matches_from(input);
    match cli::process(matches) {
        Ok(_) => panic!(),
        Err(e) => assert!(e
            .to_string()
            .starts_with(&format!("failed to open file at {}", code_path))),
    }
}

#[test]
fn encode_invalid_outputpath() {
    let version = "0";
    let name = "";
    let page_count = "1";
    let code_path = &gen_wasm_example_path();
    let output_path = "";
    let input = vec![
        "myprog",
        "encode",
        "app-template",
        version,
        name,
        page_count,
        code_path,
        output_path,
    ];
    println!("input: {:?}", input);

    let matches = cli::new_app().get_matches_from(input);
    match cli::process(matches) {
        Ok(_) => panic!(),
        Err(e) => assert!(e
            .to_string()
            .starts_with(&format!("failed to create file at {}", output_path))),
    }
}

#[test]
fn decode_invalid_datapath() {
    let data_path = "non_existing_path";
    let input = vec!["myprog", "decode", "app-template", data_path];
    println!("input: {:?}", input);

    let matches = cli::new_app().get_matches_from(input);
    match cli::process(matches) {
        Ok(_) => panic!(),
        Err(e) => assert!(e
            .to_string()
            .starts_with(&format!("failed to open file at {}", data_path))),
    }
}

fn test_encode_decode(case: AppTemplateTestCase) {
    let wasm_example_path = &gen_wasm_example_path();

    let mut wasm_example_code = Vec::new();
    File::open(&wasm_example_path)
        .unwrap()
        .read_to_end(&mut wasm_example_code)
        .unwrap();

    let tempfile_path = tempfile::NamedTempFile::new().unwrap();
    let tempfile_path = tempfile_path.path().to_str().unwrap();

    let version = "0";
    let code_path = wasm_example_path;
    let output_path = tempfile_path;
    let input = vec![
        "myprog",
        "encode",
        "app-template",
        version,
        &case.name,
        &case.page_count,
        code_path,
        output_path,
    ];
    println!("input: {:?}", input);

    let matches = cli::new_app().get_matches_from(input);
    let output = cli::process(matches).unwrap();
    println!("output: {}", output);

    let re = Regex::new(r"Wrote (\d+) bytes to (.*)").unwrap();
    let cap = re.captures(&output).unwrap();
    assert_eq!(&cap[2], output_path);

    let data_path = output_path;
    let input = vec!["myprog", "decode", "app-template", data_path];
    println!("input: {:?}", input);

    let matches = cli::new_app().get_matches_from(input);
    let output = cli::process(matches).unwrap();
    println!("output: {}", output);

    let re = Regex::new(r##"Author:(.*)...\nName: "(.*)"\nCode: (.*)\n#Pages: (\d+)"##).unwrap();
    let cap = re.captures(&output).unwrap();
    assert_eq!(&cap[2], case.name);
    assert_eq!(&cap[3], format!("{:?}", &wasm_example_code[0..4]));
    assert_eq!(&cap[4], case.page_count);
}

fn gen_wasm_example_path() -> String {
    let path = std::fs::canonicalize("../../examples/c/wasm/counter.wasm").unwrap();
    return path.to_str().unwrap().to_owned();
}
