#!/bin/sh

cd ./scripts || cd ../scripts || true
cd ..

cargo build --release
perf record --call-graph dwarf ./target/release/lazy-wfc

hotspot ./perf.data