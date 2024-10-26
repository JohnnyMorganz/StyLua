# TODO: Ensure that version is up to date
cp README.md wasm/
cp LICENSE.md wasm/
npx wasm-pack@0.10.3 build --target web --out-dir wasm/stylua.web -- --features lua52,lua53,lua54,luau

# workaround for bundler usage
echo "export { getImports as __getImports, finalizeInit as __finalizeInit }" >> wasm/stylua.web/stylua_lib.js

# bundle for node CommonJS
npx rollup@4.9.5 wasm/src/stylua_lib_node.cjs --file wasm/stylua_lib.cjs --format cjs
