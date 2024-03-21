current: actions-lua-test

actions-lua-test:
	cargo test -p pixylene-actions-lua main -- --nocapture

editor-tui-linux:
	cargo build -p pixylene-crossterm

windows:
	cargo build --target x86_64-pc-windows-gnu -p pixylene-ui
