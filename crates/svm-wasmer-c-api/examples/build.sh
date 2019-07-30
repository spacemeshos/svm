cargo +nightly build
mv ../target/debug/libsvm_wasmer_c_api.dylib ./libsvm_wasmer_c_api.dylib

make counter

pushd ./wasm
wapm run wat2wasm counter.wast
popd
