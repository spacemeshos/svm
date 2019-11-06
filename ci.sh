#!/bin/bash

# Config option

# Test thread, see README.md to see why:
export RUST_TEST_THREADS=1
# Options (Don't include --release, that added for build needing it):
export OPT="-p svm-runtime-c-api"
# Extra windows opt
export EXTRAOPT="--target x86_64-pc-windows-gnu"

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
	cargo +nightly build $OPT ; check
	cargo +nightly test $OPT ; check
elif [ "$SUITE" == "release" ]; then
  cargo +nightly build --release $OPT ; check
  cargo +nightly test --release $OPT ; check
elif [ "$SUITE" == "windowsDebug" ]; then
	installCross
	cross +nightly build $EXTRAOPT $OPT ; check
	cross +nightly test $EXTRAOPT $OPT ; check
elif [ "$SUITE" == "windowsRelease" ]; then
	installCross
	cross +nighstly build --release $EXTRAOPT $OPT; check
  cross +nightly test --release $EXTRAOPT $OPT; check
elif [ "$SUITE" == "lint" ]; then
	cargo +nightly fmt --all -- --check ; check
fi

exit 0
