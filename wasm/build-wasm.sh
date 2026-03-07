#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT=$(dirname $0)/..
CURRENT_DIR=$PROJECT_ROOT/wasm

# TODO: Ensure that version is up to date
cp $PROJECT_ROOT/README.md $CURRENT_DIR/
cp $PROJECT_ROOT/LICENSE.md $CURRENT_DIR/

cargo build --target wasm32-unknown-unknown --release --features lua52,lua53,lua54,luajit,luau,cfxlua

WASM_PATH=$PROJECT_ROOT/target/wasm32-unknown-unknown/release/stylua_lib.wasm

# install wasm-bindgen-cli matching the version in Cargo.toml
WASM_BINDGEN_VERSION=$(grep -oP 'wasm-bindgen = \{ version = "\K[^"]+' $PROJECT_ROOT/Cargo.toml)
command -v wasm-bindgen && [ "$(wasm-bindgen --version | awk '{print $2}')" = "$WASM_BINDGEN_VERSION" ] || cargo install wasm-bindgen-cli --version "$WASM_BINDGEN_VERSION"

# check wasm-bindgen version
wasm-bindgen --version

wasm-bindgen $WASM_PATH --target bundler --out-dir $CURRENT_DIR/stylua.bundler
wasm-bindgen $WASM_PATH --target nodejs --out-dir $CURRENT_DIR/stylua.node-cjs
wasm-bindgen $WASM_PATH --target experimental-nodejs-module --out-dir $CURRENT_DIR/stylua.node-esm
wasm-bindgen $WASM_PATH --target deno --out-dir $CURRENT_DIR/stylua.deno
wasm-bindgen $WASM_PATH --target web --out-dir $CURRENT_DIR/stylua.web

cp -R $CURRENT_DIR/stylua.bundler/. $CURRENT_DIR/

cp $CURRENT_DIR/stylua.node-cjs/stylua_lib.js $CURRENT_DIR/stylua_lib_node.cjs
cp $CURRENT_DIR/stylua.node-cjs/stylua_lib.d.ts $CURRENT_DIR/stylua_lib_node.d.cts

cp $CURRENT_DIR/stylua.node-esm/stylua_lib.js $CURRENT_DIR/stylua_lib_node.mjs
cp $CURRENT_DIR/stylua.node-esm/stylua_lib.d.ts $CURRENT_DIR/stylua_lib_node.d.mts

cp $CURRENT_DIR/stylua.deno/stylua_lib.js $CURRENT_DIR/stylua_lib_deno.js
cp $CURRENT_DIR/stylua.deno/stylua_lib.d.ts $CURRENT_DIR/stylua_lib_deno.d.ts

cp $CURRENT_DIR/stylua.web/stylua_lib.js $CURRENT_DIR/stylua_lib_web.js
cp $CURRENT_DIR/stylua.web/stylua_lib.d.ts $CURRENT_DIR/stylua_lib_web.d.ts

echo "Build complete!"

# Run smoke tests if --test flag is passed
if [ "$1" = "--test" ]; then
    echo "Running smoke tests..."
    cd $CURRENT_DIR && npm test
fi
