# TODO: Ensure that version is up to date
npm install -g wasm-pack
cp README.md wasm/
cp LICENSE.md wasm/
wasm-pack build --target nodejs --out-dir wasm/stylua.node -- --features lua52,lua53,lua54,luau
wasm-pack build --target bundler --out-dir wasm/stylua.bundler -- --features lua52,lua53,lua54,luau
