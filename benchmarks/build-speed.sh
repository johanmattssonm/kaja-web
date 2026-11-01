#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$script_dir"

build_package() {
    package=$1
    (cd "$package" && wasm-pack build --target web --release --out-dir pkg --no-opt)
}

build_package kaja
build_package yew
build_package leptos
build_package dioxus
