#!/bin/sh
set -e

wasm-pack build --target web --dev --out-dir pkg
python3 -m http.server 8000
