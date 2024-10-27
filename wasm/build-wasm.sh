# TODO: Ensure that version is up to date
cp README.md wasm/
cp LICENSE.md wasm/
npx wasm-pack@0.13.0 build --target web --out-dir wasm/stylua.web -- --features lua52,lua53,lua54,luau

# bundle for node CommonJS
npx rollup@4.9.5 wasm/src/stylua_lib_node.cjs --file wasm/stylua_lib.cjs --format cjs
