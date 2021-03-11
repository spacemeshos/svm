pushd crates/codec
./build.sh
popd

cp ./target/wasm32-unknown-unknown/release/svm_codec.wasm crates/codec/examples/svm_codec.wasm

pushd crates/codec/examples
npm install
npm test
popd
