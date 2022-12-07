lint:
	cargo fmt
	cargo check
	cargo clippy

build: lint
	cargo build

test: lint
	cargo test

run: build
	cargo run -- --day $(DAY) --session $(SESSION)
