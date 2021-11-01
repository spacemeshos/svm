#!/usr/bin/env bash

set -e

mkdir -p bins

if [ "${OS}" == "windows-latest"]; then
    cp target/${TARGET}/release/svm_runtime_ffi.dll bins/svm.dll
    cp target/${TARGET}/release/svm_runtime_ffi.lib bins/svm.lib
    cp target/${TARGET}/release/svm_runtime_ffi.pcb bins/svm.pcb
    cp target/${TARGET}/release/svm-cli.exe bins/svm-cli.exe
    cp target/${TARGET}/release/svm.h bins/svm.h
elif [ "${OS}" == "macos-latest"]; then
    cp target/${TARGET}/release/libsvm_runtime_ffi.dylib bins/libsvm.dylib
    cp target/${TARGET}/release/libsvm_runtime_ffi.a bins/libsvm.a
    cp target/${TARGET}/release/svm-cli bins/svm-cli
    cp target/${TARGET}/release/svm.h bins/svm.h
elif [ "${OS}" == "ubuntu-latest"]; then
    cp target/${TARGET}/release/libsvm_runtime_ffi.so bins/libsvm.so
    cp target/${TARGET}/release/libsvm_runtime_ffi.a bins/libsvm.a
    cp target/${TARGET}/release/svm-cli bins/svm-cli
    cp target/${TARGET}/release/svm.h bins/svm.h
fi
