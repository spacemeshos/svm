# abort on error
set -e

cargo +nightly build --release

mv ../../../target/release/libsvm_runtime_c_api.dylib ./libsvm_wasmer_c_api.dylib

make counter

pushd ./wasm
wapm run wat2wasm counter.wast
popd

./counter.out
