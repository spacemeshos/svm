use svm_cli::wasm_value;
use svm_types::Address;

pub fn fmt_buf(hex: &str) -> String {
    format!(
        "{:?}",
        hex::decode(hex).unwrap().iter().take(4).collect::<Vec<_>>()
    )
}

pub fn fmt_addr(addr_hex: &str) -> String {
    let addr = hex::decode(addr_hex).unwrap();
    let addr = Address::from(addr.as_slice());
    addr.fmt(4, 4, " ")
}

pub fn fmt_args(args: Vec<String>) -> String {
    format!(
        "{:?}",
        args.iter()
            .map(|v| wasm_value::parse_str(v).unwrap())
            .collect::<Vec<_>>()
    )
}
