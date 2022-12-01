lint:
	cargo fmt
	cargo check
	cargo clippy

build: lint
	cargo build

run: build
ifndef AOC_DAY
	cargo run
else
	cargo run -- --day $(AOC_DAY)
endif
