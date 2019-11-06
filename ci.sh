#!/bin/bash

# This function aim to interupt the build if a step fail.
check()
{
	if [ $? != 0 ]; then
		exit 1
	fi
}

# This function install cross
installCross()
{
	# Installing cross
	cargo install cross ; check
}

# Debug and Release are the default *nix friendly ci (used by linux and osx build).
# Windows* use docker and cross to build and test in a windows docker in linux.
if [ "$SUITE" == "debug" ]; then
	cargo +nightly build -p svm-runtime-c-api ; check
	RUST_TEST_THREADS=1 cargo +nightly test -p svm-runtime-c-api ; check
elif [ "$SUITE" == "release" ]; then
  cargo +nightly build -p svm-runtime-c-api --release ; check
  RUST_TEST_THREADS=1 cargo +nightly test -p svm-runtime-c-api --release ; check
elif [ "$SUITE" == "windowsDebug" ]; then
	installCross
	cross +nightly build --all --target x86_64-pc-windows-gnu ; check
	RUST_TEST_THREADS=1 cross +nightly test --all --target x86_64-pc-windows-gnu ; check
elif [ "$SUITE" == "windowsRelease" ]; then
	installCross
	cross +nightly build --all --release --target x86_64-pc-windows-gnu ; check
  RUST_TEST_THREADS=1 cross +nightly test --all --release --target x86_64-pc-windows-gnu ; check
elif [ "$SUITE" == "lint" ]; then
	# Install rustfmt
	rustup component add rustfmt --toolchain nightly ; check
	cargo +nightly fmt --all -- --check ; check
fi

exit 0
