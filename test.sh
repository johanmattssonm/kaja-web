#!/bin/bash
set -e

wasm-pack test --headless --chrome --test alltests
