./../build.sh 
cp ../../../../target/wasm32-unknown-unknown/release/svm_codec.wasm svm_codec.wasm

npm install
npm test
