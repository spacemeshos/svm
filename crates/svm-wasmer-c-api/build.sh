cargo +nightly build

mv ./target/debug/libsvm_wasmer_c_api.dylib ./examples/libsvm_wasmer_c_api.dylib

pushd ./examples/wasm
wapm run wat2wasm counter.wast
popd
