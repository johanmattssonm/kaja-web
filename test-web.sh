#!/bin/bash
set -e

wasm-pack test --chrome --test alltests
