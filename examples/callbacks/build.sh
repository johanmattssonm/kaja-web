#!/bin/sh

set -e
cargo build

wasm-pack build \
    --release \
    --target web \
    --out-dir pkg \
    --out-name clickcounter \
    --no-typescript

cp index.html pkg/
cp main.js pkg/
cp styles.css pkg/
