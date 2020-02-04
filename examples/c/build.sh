# abort on error
set -e

pushd ../../crates/svm-runtime-c-api
cargo +nightly build --release
popd

unameOut="$(uname -s)"
case "${unameOut}" in
  Linux*) ext=so;;
  Darwin*) ext=dylib;;
  *) ext=invalid;;
esac

mv ../../crates/svm-runtime-c-api/target/release/libsvm_runtime_c_api.${ext} ./svm.${ext}

make counter

pushd ./wasm
rm -f counter.wasm
wapm run wat2wasm counter.wast
popd

./counter.out
