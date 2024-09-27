cargo make build-web
wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/debug/cozy_corner.wasm

echo "Build Completed"