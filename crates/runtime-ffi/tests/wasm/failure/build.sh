cargo +nightly build --features=ffi,static-alloc --no-default-features --release --target wasm32-unknown-unknown 

if [ -f ./../failure.wasm ]; then
    rm ./../failure.wasm
fi

mv ./target/wasm32-unknown-unknown/release/svm_runtime_examples_failure.wasm ./../failure.wasm
