#!/bin/bash
# Pour lancer le serveur, mettre en argument l'ip/hostname
# du serveur
if [ "$#" -eq 0  ]; then
    echo "You need to pass the ip/hostname of the server"
    exit 1;
fi
export PATH=$(pwd)/target/release:$PATH
cargo build --release --bin blobwar_iterative_deepening
cargo run --release --bin client "$1"
