use svm_abi_encoder::Encoder;

pub fn set_returndata(data: &dyn Encoder) {
    let mut w: Vec<u8> = Vec::new();

    data.encode(&mut w);
}
