#!/bin/sh

# cargo install wasm-pack

cd $(dirname $0)

export WASM_BINDGEN_WEAKREF=1

# npm run build
wasm-pack build --release --target web

