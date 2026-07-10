#!/bin/sh
set -eu

mkdir -p pkg

CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-Ctarget-feature=+atomics -Clink-args=--shared-memory -Clink-args=--max-memory=1073741824 -Clink-args=--import-memory -Clink-args=--export=__heap_base -Clink-args=--export=__wasm_init_tls -Clink-args=--export=__tls_size -Clink-args=--export=__tls_align -Clink-args=--export=__tls_base" cargo +nightly build -Zbuild-std=std,panic_abort --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/raytrace_parallel.wasm --target no-modules --out-dir ./pkg

cp static/index.html ./pkg/
cp static/worker.js ./pkg/
cp static/index.js ./pkg/
