#!/usr/bin/env bash

set -ex


PROJECT_NAME='webgl_test'

cargo +nightly build --target wasm32-unknown-unknown --release
wasm-bindgen                                                     \
    ./target/wasm32-unknown-unknown/release/"$PROJECT_NAME".wasm \
    --out-dir out
cd out
if [ "$1" == "fresh" ]
then
    npm install
fi
tslint -p tsconfig.json
tsc
wasm-gc ./"$PROJECT_NAME"_bg.wasm
wasm-opt -O4 ./"$PROJECT_NAME"_bg.wasm
webpack
cp index.html ./dist
cp style.css ./dist
cp -a img ./dist
cd ..
