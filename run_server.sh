#!/bin/bash
# Script to run automatically the server
export PATH=$(pwd)/target/release:$PATH
cargo build --release --bin blobwar_iterative_deepening
cargo run --release --bin server
