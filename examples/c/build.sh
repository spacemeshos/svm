# abort on error
set -e

pushd ../../crates/svm-runtime-c-api
# a build side-effect will to generate `svm.h` under `target/release`.
cargo +nightly build --release
popd

# figuring out the svm object file extension by platform
unameOut="$(uname -s)"
case "${unameOut}" in
  Linux*) ext=so;;
  Darwin*) ext=dylib;;
  *) ext=invalid;;
esac

# copying the `svm` object/header files under `examples`.
cp ../../target/release/libsvm_runtime_c_api.${ext} ../svm.${ext}
cp ../../target/release/svm.h ../svm.h

make counter

#pushd ./wasm
#rm -f counter.wasm
#wapm run wat2wasm counter.wast
#popd

./counter.out
