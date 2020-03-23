use regex::Regex;
use std::fs::File;
use std::io::Read;
use svm_cli::cli;

struct AppTemplateTestCase {
    version: String,
    name: String,
    page_count: String,
}

const WASM_EXAMPLE_PATH: &'static str = "../../examples/c/wasm/counter.wasm";

#[test]
fn encode_decode() {
    let cases = vec![
        AppTemplateTestCase {
            version: String::from("0"),
            name: String::from("my name"),
            page_count: String::from("0"),
        },
        AppTemplateTestCase {
            version: String::from("1"),
            name: String::from("שלום"),
            page_count: String::from("1000"),
        },
        AppTemplateTestCase {
            version: String::from("1"),
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
        "app_template",
        version,
        name,
        page_count,
        code_path,
        output_path,
    ];

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
    let code_path = WASM_EXAMPLE_PATH;
    let output_path = "";
    let input = vec![
        "myprog",
        "encode",
        "app_template",
        version,
        name,
        page_count,
        code_path,
        output_path,
    ];

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
    let input = vec!["myprog", "decode", "app_template", data_path];

    let matches = cli::new_app().get_matches_from(input);
    let res = cli::process(matches);
    assert!(res.is_err());
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .starts_with(&format!("failed to open file at {}", data_path)));
}

fn test_encode_decode(case: AppTemplateTestCase) {
    let mut wasm_example_code = Vec::new();
    File::open(WASM_EXAMPLE_PATH)
        .unwrap()
        .read_to_end(&mut wasm_example_code)
        .unwrap();

    let tempfile_path = tempfile::NamedTempFile::new().unwrap();
    let tempfile_path = tempfile_path.path().to_str().unwrap();

    let code_path = WASM_EXAMPLE_PATH;
    let output_path = tempfile_path;
    let input = vec![
        "myprog",
        "encode",
        "app_template",
        &case.version,
        &case.name,
        &case.page_count,
        code_path,
        output_path,
    ];
    let matches = cli::new_app().get_matches_from(input);
    let output = cli::process(matches).unwrap();

    let re = Regex::new(r"Wrote (\d+) bytes to (.*)").unwrap();
    let cap = re.captures(&output).unwrap();
    assert_eq!(&cap[2], output_path);

    let data_path = output_path;
    let input = vec!["myprog", "decode", "app_template", data_path];
    let matches = cli::new_app().get_matches_from(input);
    let output = cli::process(matches).unwrap();

    let re = Regex::new(r"Version: (.*)\nName: (.*)\nCode: (.*)\n#Pages: (\d+)").unwrap();
    let caps = re.captures(&output).unwrap();
    assert_eq!(&caps[1], case.version);
    assert_eq!(&caps[2], case.name);
    assert_eq!(&caps[3], format!("{:?}", &wasm_example_code[0..4]));
    assert_eq!(&caps[4], case.page_count);
}
