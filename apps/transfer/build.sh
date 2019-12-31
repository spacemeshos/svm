cargo +nightly build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ../../apps/precompiled
pushd ../precompiled
wapm run wasm2wat transfer.wasm > transfer.wast
popd
