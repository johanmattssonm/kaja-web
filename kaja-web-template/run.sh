#!/bin/sh
set -e

wasm-pack build --target web --release --out-dir pkg
python3 -m http.server 8000
