#!/bin/sh
set -e

wasm-pack build \
    --release \
    --target web \
    --out-dir pkg \
    --out-name main

cp index.html pkg/
cp main.js pkg/
cp styles.css pkg/
