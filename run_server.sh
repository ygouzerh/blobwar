# Script to run automatically the server
export PATH="$(pwd)/target/realease":$PATH
cargo build --release --bin blobwar_iterative_deepening
cargo run --release --bin server
