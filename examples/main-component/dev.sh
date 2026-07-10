#!/bin/sh
set -e

wasm-pack build \
    --debug \
    --target web \
    --out-dir pkg \
    --out-name componentexample

cp index.html pkg/
cp loader.js pkg/
cp styles.css pkg/
