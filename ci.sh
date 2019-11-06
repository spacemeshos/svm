#!/bin/bash

# Config option

# Test thread, see README.md to see why:
export RUST_TEST_THREADS=1
# Options (Don't include --release, that added for build needing it):
# To build only one package change this to "-p <package>" here.
export OPT="--all"
# Extra windows opt.
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

# This function upload Artifacts if needed
upload()
{
	if ([ "$TRAVIS_BRANCH" == "master" ] || [ "$TRAVIS_BRANCH" == "develop" ]) && [ "$TRAVIS_EVENT_TYPE" != "pull_request" ]; then
		# Add all file path to upload to upload.txt
		ls ./target/debug/libsvm_runtime_c_api.{so,dylib,dll,h} > upload.txt
	fi
}


# This file contains file path to upload, ensure that is at least empty.
touch upload.txt

# Debug and Release are the default *nix friendly ci (used by linux and osx build).
# Windows* use docker and cross to build and test in a windows docker in linux.
if [ "$SUITE" == "debug" ]; then
	cargo +nightly build $OPT ; check
	if [ "$TEST" == "True" ]; then cargo +nightly test $OPT ; check ; fi
	upload
elif [ "$SUITE" == "release" ]; then
  cargo +nightly build --release $OPT ; check
	if [ "$TEST" == "True" ]; then cargo +nightly test --release $OPT ; check ; fi
	upload
elif [ "$SUITE" == "windowsDebug" ]; then
	installCross
	cross +nightly build $EXTRAOPT $OPT ; check
	if [ "$TEST" == "True" ]; then cross +nightly test $EXTRAOPT $OPT ; check ; fi
	upload
elif [ "$SUITE" == "windowsRelease" ]; then
	installCross
	cross +nightly build --release $EXTRAOPT $OPT; check
  if [ "$TEST" == "True" ]; then cross +nightly test --release $EXTRAOPT $OPT; check ; fi
	upload
elif [ "$SUITE" == "lint" ]; then
	cargo +nightly fmt $OPT -- --check ; check
fi

exit 0
