.PHONY: rebuild check run

rebuild:
	touch src/main.rs

check:
	clear
	cargo check

run:
	clear
	cargo run
