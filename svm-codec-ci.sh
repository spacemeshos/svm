pushd crates/svm-codec
./build.sh
popd

cp ./target/wasm32-unknown-unknown/release/svm_codec.wasm crates/svm-codec/examples/svm_codec.wasm

pushd crates/svm-codec/examples
npm install
npm test
popd
