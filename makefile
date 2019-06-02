
DBG_FLAGS=RUST_BACKTRACE=1 RUST_LOG=coat=debug

check:
	cargo check

check-dev:
	cargo check --example dev

build:
	cargo build

dev:
	$(DBG_FLAGS) cargo run --example dev

widgets:
	$(DBG_FLAGS) cargo run --example widgets

