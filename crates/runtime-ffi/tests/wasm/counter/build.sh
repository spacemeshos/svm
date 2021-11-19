cargo +nightly build --release --target wasm32-unknown-unknown

if [ -f ./../counter.wasm ]; then
    rm ./../counter.wasm
fi

mv ./target/wasm32-unknown-unknown/release/svm_runtime_examples_counter.wasm ./../counter.wasm
