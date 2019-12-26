# abort on error
set -e

cargo +nightly build --release

unameOut="$(uname -s)"
case "${unameOut}" in
  Linux*) ext=so;;
  Darwin*) ext=dylib;;
  *) ext=invalid;;
esac

mv ../../../target/release/libsvm_runtime_c_api.${ext} ./svm.${ext}

make counter

pushd ./wasm
wapm run wat2wasm counter.wast
popd

./counter.out
