# TODO: Ensure that version is up to date
cp README.md wasm/
cp LICENSE.md wasm/
wasm-pack build --target nodejs --out-dir wasm/stylua.node
wasm-pack build --target bundler --out-dir wasm/stylua.bundler
