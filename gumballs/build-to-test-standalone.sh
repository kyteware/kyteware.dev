cargo build --bin gumballs-standalone --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./standalone_build/ --out-name "gumball-standalone" ./target/wasm32-unknown-unknown/release/gumballs-standalone.wasm
python -m http.server --directory ./standalone_build