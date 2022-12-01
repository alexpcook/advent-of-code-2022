lint:
	cargo fmt
	cargo check
	cargo clippy

build: lint
	cargo build

run: build
	cargo run -- --day $(DAY) --session $(SESSION)
