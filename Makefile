current: actions-lua-test

actions-lua-test:
	cd src; \
		cargo test -p pixylene-actions-lua main -- --nocapture

editor-tui-linux:
	cd src; \
		cargo build -p pixylene-crossterm
