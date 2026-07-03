#!/bin/sh
set -e

wasm-pack build \
    --release \
    --target web \
    --out-dir pkg \
    --out-name componentexample

cp index.html pkg/
cp main.js pkg/
cp styles.css pkg/
