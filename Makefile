current: actions-lua-test

actions-lua-test:
	cargo test -p pixylene-lua main -- --nocapture

editor-tui-linux:
	cargo build -p pixylene-crossterm
