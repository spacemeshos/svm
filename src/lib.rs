//! # The Spacemesh Virtual Machine (SVM)
//!
//! Much like Ethereum, Spacemesh is a distributed state machine, rather than
//! simply a ledger. The SVM is responsible for managing state and executing
//! smart contracts within the Spacemesh architecture and it integrates closely
//! with [`go-spacemesh`](https://github.com/spacemeshos/go-spacemesh) via a C
//! API.
//!
//! The SVM is written in Rust as a collection of fully open-source crates.
//! These crates are then brought together and available under the top level
//! `svm` Rust crate. Spacemesh smart contracts must be written in smWasm,
//! which is a strict subset of WebAssembly; it is possible to write Rust code
//! and compile it down to smWasm thanks to the Rust SDK that SVM ships with.

extern crate svm_codec;
extern crate svm_gas;
extern crate svm_layout;
extern crate svm_query;
extern crate svm_runtime;
extern crate svm_runtime_ffi;
extern crate svm_sdk;
extern crate svm_types;
