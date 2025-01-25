build:
	cargo build --release --target wasm32-wasip1

test:
	cargo nextest run

test-watch:
	bacon nextest
