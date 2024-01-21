# NB: the project will be built if make is invoked without any arguments.
.PHONY: default
default: build

.PHONY: build
build:
	cargo build

.PHONY: check
check:
	cargo check

.PHONY: clean
clean:
	cargo clean

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: fmt-check
fmt-check:
	cargo fmt --all -- --check

.PHONY: clippy
clippy:
	cargo clippy --all-targets --all-features --tests -- -D warnings

.PHONY: postgres
postgres:
	./bin/local_postgres.sh start

.PHONY: test
test:
	cargo test --all --workspace --bins --tests --benches