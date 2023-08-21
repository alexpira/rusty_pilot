#!/bin/sh

cd $(dirname $0)

# wasm-pack build --release --target web && \
python3 ./serve.py

