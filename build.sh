cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/ld_framework.wasm --out-dir web_build --out-name ld --target web --no-typescript