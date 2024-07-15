

tui:
	cargo build -p pixylene-ui --bin pixylenetui -F tui${LUA} --release

cli:
	cargo build -p pixylene-ui --bin pixylenecli --release

gui:
	cargo build -p pixylene-ui --bin pixylenegui -F minifb${LUA} --release

web:
	cargo build -p pixylene-ui -F wasm${LUA} --bin pixyleneweb --target wasm32-unknown-emscripten --release && \
	wasm-bindgen ./target/wasm32-unknown-unknown/release/pixyleneweb.wasm --out-dir ./pixylene-ui/pkg/ --target web

lua-docs:
	cargo run -p pixylene-lua -F docs --release -- ./assets/docs/pixylene-lua/pixylene-lua.json && \
	cd ./assets/docs/pixylene-lua && \
	~/.cargo/bin/tealr_doc_gen run
