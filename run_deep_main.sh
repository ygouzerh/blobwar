#!/bin/bash
# Script to run automatically without optimization the main with deepening
export PATH=$(pwd)/target/release:$PATH
cargo build --release --bin blobwar_iterative_deepening
cargo run --release --bin blobwar
