default: check

build-ios:


build-unffi:
	cargo run --package uniffi-bindgen --bin uniffi-bindgen generate ./target/release/libsdkrs_uniffi.dylib --library --language swift --language kotlin --no-format --out-dir tmp/bindings

# Install the wasm target
install-rustc-toolchain:
	rustup component add rustfmt clippy
	cargo install --locked cargo-deny

test:
	cargo nextest run

audit:
	cargo deny --all-features check licenses

check:
	cargo check --release

check-docs:
	cargo doc --no-deps

fmt:
	cargo fmt --all

lint:
	cargo clippy

fix:
	cargo clippy --fix --allow-dirty
