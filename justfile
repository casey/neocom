default: test

test:
	cargo test

watch:
	cargo watch -x check

life:
	cd neocom-special && cargo run --example life
