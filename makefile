build:
	cargo build --release

local-deploy:
	cp target/release/kiss ~/.local/bin/
