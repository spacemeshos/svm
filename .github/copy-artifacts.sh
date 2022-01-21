#!/usr/bin/env bash

set -e

mkdir -p bins

if [[ "${OS}" == "windows-latest" ]]; then
    cp target/release/svm_runtime_ffi.dll bins/svm.dll
    cp target/release/svm_runtime_ffi.lib bins/svm.lib
    cp target/release/svm_runtime_ffi.pdb bins/svm.pdb
    cp target/release/svm-cli.exe bins/svm-cli.exe
    cp target/release/svm.h bins/svm.h
elif [[ "${OS}" == "macos-latest" ]]; then
    mkdir -p bins/x86_64
    mkdir -p bins/aarch64
    cp target/aarch64-apple-darwin/release/libsvm_runtime_ffi.dylib bins/aarch64/libsvm.dylib
    cp target/aarch64-apple-darwin/release/libsvm_runtime_ffi.a bins/aarch64/libsvm.a
    cp target/aarch64-apple-darwin/release/svm-cli bins/aarch64/svm-cli
    cp target/release/libsvm_runtime_ffi.dylib bins/x86_64/libsvm.dylib
    cp target/release/libsvm_runtime_ffi.a bins/x86_64/libsvm.a
    cp target/release/svm-cli bins/x86_64/svm-cli
    cp target/release/svm.h bins/svm.h
elif [[ "${OS}" == "ubuntu-latest" ]]; then
    cp target/release/libsvm_runtime_ffi.so bins/libsvm.so
    cp target/release/libsvm_runtime_ffi.a bins/libsvm.a
    cp target/release/svm-cli bins/svm-cli
    cp target/release/svm.h bins/svm.h
fi
