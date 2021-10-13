cargo +nightly build --features=ffi,static-alloc,meta --no-default-features --release --target wasm32-unknown-unknown

rm -f craft_deploy_example.wasm
cp ./target/wasm32-unknown-unknown/release/svm_cli_craft_deploy_example.wasm ./craft_deploy_example.wasm

./../../../../target/debug/svm-cli craft-deploy --smwasm craft_deploy_example.wasm --meta Template-meta.json --output template_example.svm
