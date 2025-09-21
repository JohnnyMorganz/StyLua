# TODO: Ensure that version is up to date
cp README.md wasm/
cp LICENSE.md wasm/

# Build WASM with wasm-pack
npx wasm-pack@0.10.3 build --target web --out-dir wasm/stylua.web -- --features lua52,lua53,lua54,luajit,luau,cfxlua

# workaround for bundler usage
echo "export { __wbg_get_imports as __getImports, __wbg_finalize_init as __finalizeInit }" >> wasm/stylua.web/stylua_lib.js

# bundle for node CommonJS
cd wasm && ./node_modules/.bin/rollup src/stylua_lib_node.mjs --file stylua_lib.cjs --format cjs \
  --external 'node:fs,node:path,node:url' --plugin @rollup/plugin-node-resolve --plugin @rollup/plugin-commonjs
cd ..

# bundle for node ESM
cd wasm && ./node_modules/.bin/rollup src/stylua_lib_node.mjs --file stylua_lib.mjs --format es \
  --external 'node:fs,node:path,node:url'
cd ..
