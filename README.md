## SVM (Spacemesh Virtual Machine)

This repository will implement the _Spacemesh_ smart contracts vm.


### Project Goals
* Self-contained. Should be hosted by the [Spacemesh Golang full-node](https://github.com/spacemeshos/go-spacemesh) and future _Spacemesh_ Rust full-node
* Built on top of [wasmer](https://wasmer.io)
* Future compatible for the _SMESH_ programming-language (the language is still in a research stage).


### Rust
`SVM` depends on the single-pass compiler of `wasmer` which requires the `rust nightly` release channel.
More information about installing `rust nightly` can be [found here](https://doc.rust-lang.org/1.2.0/book/nightly-rust.html)


### Build & Tests
The `svm` project is organized into a couple of crates.
The root crate is called `svm` and it's a workspace crate.

In order to build the `svm` project crates at once and run their tests:
```rust
cargo +nightly build
RUST_TEST_THREADS=1 cargo +nightly test --all
```

If you want to build & test in release mode execute:
```rust
cargo +nightly build --release
RUST_TEST_THREADS=1 cargo +nightly test --all --release
```

Note: since `leveldb` and `rocksdb` can't have parallel isolated connections to the same database (directory).
That's why we ask `cargo` to run the tests serially (`RUST_TEST_THREADS=1`).


### Got Questions?
- Introduce yourself and ask anything on the [spacemesh gitter channel](https://gitter.im/spacemesh-os/Lobby).
- DM [@teamspacemesh](https://twitter.com/teamspacemesh)
