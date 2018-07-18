#!/usr/bin/env bash

set -ex


PROJECT_NAME='webgl_test'

cargo +nightly build --target wasm32-unknown-unknown --release
../wasm-bindgen/target/release/wasm-bindgen                    \
    ./target/wasm32-unknown-unknown/debug/"$PROJECT_NAME".wasm \
    --out-dir out
cd out
npm install
wasm-gc ./"$PROJECT_NAME"_bg.wasm
wasm-opt -O4 ./"$PROJECT_NAME"_bg.wasm
cd ..
