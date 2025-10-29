CARGO_MANIFEST_DIR := $(shell pwd)

dev:
	BEVY_ASSET_ROOT=$(CARGO_MANIFEST_DIR) CARGO_MANIFEST_DIR=$(CARGO_MANIFEST_DIR) dx serve --hot-patch --features bevy/dynamic_linking
	
run_with_dynamic_link:
	cargo run --features bevy/dynamic_linking

build:
	cargo build

fmt:
	cargo fmt
	cargo fix --allow-dirty
