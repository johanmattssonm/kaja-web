#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$script_dir"

build_and_measure() {
    package=$1
    wasm_name=$2

    (cd "$package" && wasm-pack build --target web --release --out-dir pkg)
    wasm_file="$package/pkg/${wasm_name}_bg.wasm"
    bytes=$(wc -c < "$wasm_file" | tr -d ' ')
    printf '%-8s %s bytes\n' "$package" "$bytes"
}

printf 'Uncompressed wasm-bindgen output (release, bytes):\n'
build_and_measure kaja kaja_component_benchmark
build_and_measure yew yew_component_benchmark
build_and_measure leptos leptos_component_benchmark
build_and_measure dioxus dioxus_component_benchmark
