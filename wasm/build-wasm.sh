# TODO: Ensure that version is up to date
cp README.md wasm/
cp LICENSE.md wasm/
npx wasm-pack@0.10.3 build --target nodejs --out-dir wasm/stylua.node -- --features lua52,lua53,lua54,luau
npx wasm-pack@0.10.3 build --target bundler --out-dir wasm/stylua.bundler -- --features lua52,lua53,lua54,luau
