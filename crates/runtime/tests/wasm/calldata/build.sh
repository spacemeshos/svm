cargo +nightly build --release --target wasm32-unknown-unknown --features=ffi  

if [ -f ./../runtime_calldata.wasm ]; then
    rm ./../runtime_calldata.wasm
fi

mv ./target/wasm32-unknown-unknown/release/svm_runtime_examples_calldata.wasm ./../runtime_calldata.wasm 