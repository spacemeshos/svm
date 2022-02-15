cargo +nightly build --release --target wasm32-unknown-unknown 

if [ -f simple_coin_transfer_template.wasm ]; then
    rm simple_coin_transfer_template.wasm
fi

mv ./target/wasm32-unknown-unknown/release/simple_coin_transfer_template.wasm simple_coin_transfer_template.wasm
