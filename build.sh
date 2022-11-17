#!/bin/bash

#wasm-pack build --target no-modules
wasm-pack build $@ --target web

# cargo build --target wasm32-unknown-unknown
# cd target/wasm32-unknown-unknown/debug
# wasm-bindgen --target web --no-typescript --out-dir . duchesslib.wasm
# wasm-gc duchesslib_bg.wasm
# cp duchesslib_bg.wasm duchesslib.js ../../../
# cd ../../../
