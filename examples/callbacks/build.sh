#!/bin/sh
set -e

wasm-pack build \
    --release \
    --target web \
    --out-dir pkg \
    --out-name main \
    --no-typescript

cp index.html pkg/
cp loader.js pkg/
cp styles.css pkg/
