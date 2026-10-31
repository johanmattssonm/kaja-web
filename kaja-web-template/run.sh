#!/bin/sh
set -e

wasm-pack build --target web --release --out-dir pkg
echo "Open http://localhost:8000 and check the browser console for the Kaja Web startup message."
python3 -m http.server 8000
