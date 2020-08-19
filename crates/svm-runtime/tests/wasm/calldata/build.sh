cargo +nightly build --release --target wasm32-unknown-unknown 

mv ../../../../../target/wasm32-unknown-unknown/release/svm_runtime_examples_calldata.wasm ./../runtime_calldata.wasm
