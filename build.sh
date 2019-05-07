#!/bin/sh

cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen --out-dir src --out-name bindings target/wasm32-unknown-unknown/release/wasm_bindgen_btreemap_tuple_key.wasm
yarn tsc
