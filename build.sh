#!/bin/sh

set -ex

cargo build --target $TARGET --all-features --release

if [ -z $DISABLE_EXAMPLES ]; then
	cargo build --target $TARGET --all-features --examples
fi

cargo deadlinks --ignore-fragments
