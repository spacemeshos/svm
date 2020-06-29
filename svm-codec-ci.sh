cd crates/svm-codec
./build.sh
cd ..
cd ..

cp ./target/wasm32-unknown-unknown/release/svm_codec.wasm svm_codec.wasm

npm install
npm test crates/svm-codec/examples/test.js 
