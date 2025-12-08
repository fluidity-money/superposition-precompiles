#!/bin/sh -e

cargo test --features std

make

arbos-forge test $@
