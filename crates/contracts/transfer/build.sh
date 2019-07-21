cargo +nightly build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ../../contracts/wasm
pushd ../wasm
wapm run wasm2wat transfer.wasm > transfer.wast
popd
