#!/bin/sh
set -eu

./build.sh

python3 server.py
