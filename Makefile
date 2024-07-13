current: actions-lua-test

actions-lua-test:
	cargo test -p pixylene-lua main -- --nocapture

web:
	cargo build -p pixylene-ui -F wasm --bin pixyleneweb --target wasm32-unknown-unknown --release && \
	wasm-bindgen ./target/wasm32-unknown-unknown/release/pixyleneweb.wasm --out-dir ./pixylene-ui/pkg/ --target web
