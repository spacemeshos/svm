cargo +nightly build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ../../../apps/wasm
pushd ../../wasm
wasm2wat wallet.wasm > wallet.wast
rm wallet.wasm
wat2wasm wallet.wast
popd
