cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./web_resource_out/ --out-name "gumball" ./target/wasm32-unknown-unknown/release/exploder.wasm
python -m http.server --directory web_resource_out