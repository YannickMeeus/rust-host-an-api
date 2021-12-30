MAKEFLAGS += --silent

.PHONY: dev
dev:
	cargo watch -x run

.PHONY: install-dev-deps
install-dev-deps:
	# Watch is used as part of the dev experience.
	## https://crates.io/crates/cargo-watch
	cargo install cargo-watch 