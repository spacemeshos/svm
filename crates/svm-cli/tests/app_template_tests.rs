use regex::Regex;
use svm_cli::cli;
use svm_cli::common::{decode_hex, write_to_file};

struct AppTemplateTestCase {
    version: String,
    name: String,
}

const WASM_CODE: &'static str = "0061736d0100000001170460047f7f7f7f0060037f7f7f017f60017f006000017f024a040373766d1473746f726167655f77726974655f6933325f6c6500000373766d1373746f726167655f726561645f6933325f6c65000103656e7603696e63000203656e760367657400030305040203020305030100010733040b73746f726167655f696e6300040b73746f726167655f676574000508686f73745f696e63000608686f73745f67657400070a28040f0041004100100520006a410410000b0a0041004100410410010b0600200010020b040010030b";

#[test]
fn encode_decode() {
    let cases = vec![
        AppTemplateTestCase {
            version: String::from("0"),
            name: String::from("my name"),
        },
        AppTemplateTestCase {
            version: String::from("1"),
            name: String::from("שלום"),
        },
        AppTemplateTestCase {
            version: String::from("1"),
            name: String::from("नमस्ते"),
        },
    ];

    for case in cases {
        test_encode_decode(case)
    }
}

fn test_encode_decode(case: AppTemplateTestCase) {
    let wasm_code = decode_hex(WASM_CODE).unwrap();
    let code_path = tempfile::NamedTempFile::new().unwrap();
    let code_path = code_path.path().to_str().unwrap();
    write_to_file(code_path, &wasm_code).unwrap();

    let output_path = tempfile::NamedTempFile::new().unwrap();
    let output_path = output_path.path().to_str().unwrap();

    let input = vec![
        "myprog",
        "encode",
        "app_template",
        &case.version,
        &case.name,
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

    let re = Regex::new(r"Version: (.*)\nName: (.*)\nCode: (.*)\n").unwrap();
    let caps = re.captures(&output).unwrap();
    assert_eq!(&caps[1], case.version);
    assert_eq!(&caps[2], case.name);
    assert_eq!(&caps[3], format!("{:?}", &wasm_code[0..4]));
}

#[test]
fn encode_invalid_codepath() {
    let version = "0";
    let name = "";
    let code_path = "invalid_codepath";
    let output_path = "";
    let input = vec![
        "myprog",
        "encode",
        "app_template",
        version,
        name,
        code_path,
        output_path,
    ];

    let matches = cli::new_app().get_matches_from(input);
    let res = cli::process(matches);
    assert!(res.is_err());
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .starts_with(&format!("failed to open file at {}", code_path)));
}

#[test]
fn encode_invalid_outputpath() {
    let wasm_code = decode_hex(WASM_CODE).unwrap();
    let code_path = tempfile::NamedTempFile::new().unwrap();
    let code_path = code_path.path().to_str().unwrap();
    write_to_file(code_path, &wasm_code).unwrap();

    let version = "0";
    let name = "";
    let output_path = "";
    let input = vec![
        "myprog",
        "encode",
        "app_template",
        version,
        name,
        code_path,
        output_path,
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
