cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/ld_framework.wasm --out-dir web_build --out-name ld --target web --no-typescript