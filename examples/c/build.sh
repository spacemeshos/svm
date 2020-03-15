# abort on error
set -e

pushd ../../crates/svm-runtime-c-api
# a build side-effect will to generate `svm.h` under `examples`.
cargo +nightly build --release
popd

# figuring out the svm object file extension by platform
unameOut="$(uname -s)"
case "${unameOut}" in
  Linux*) ext=so;;
  Darwin*) ext=dylib;;
  *) ext=invalid;;
esac

# copying the `svm` objectfile under `examples`.
mv ../../target/release/libsvm_runtime_c_api.${ext} ../svm.${ext}

make counter

pushd ./wasm
rm -f counter.wasm
wapm run wat2wasm counter.wast
popd

./counter.out
