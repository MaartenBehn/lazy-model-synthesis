#!/bin/sh

cd ./scripts || cd ../scripts || true
cd ..

export CARGO_PROFILE_DEV_OPT_LEVEL=3
cargo build
perf record --call-graph dwarf ./target/debug/lazy-wfc

hotspot ./perf.data