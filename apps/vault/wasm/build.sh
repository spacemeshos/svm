cargo +nightly build --release --target wasm32-unknown-unknown 

cp ../../../target/wasm32-unknown-unknown/release/svm_app_vault_wasm.wasm vault.wasm 

# wasm2wat vault.wasm > vault.wast

# wat2wasm vault.wast
