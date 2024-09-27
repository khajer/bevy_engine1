# build to wasm

target wasm
```
rustup target add wasm32-unknown-unknown
```

runserver 
```
cargo make run-web
```

binding to web
```
cargo make build-web

wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/debug/cozy_corner.wasm
```