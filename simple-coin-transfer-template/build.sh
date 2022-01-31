cargo +nightly build --release --target wasm32-unknown-unknown 

cp ./target/wasm32-unknown-unknown/release/svm_simple_coin_transfer.wasm ./simple_coin_transfer.wasm
