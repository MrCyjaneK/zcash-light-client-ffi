SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:

APPLE_TRIPLETS = aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-darwin x86_64-apple-darwin

.PHONY: install_deps
install_toolchain:
	rustup toolchain add stable
	rustup +stable target add aarch64-apple-ios x86_64-apple-ios x86_64-apple-darwin aarch64-apple-darwin aarch64-apple-ios-sim
	rustup +stable target add aarch64-linux-android x86_64-linux-android armv7-linux-androideabi
	cargo install cargo-ndk

.PHONY: clean
clean:
	rm -rf products
	rm -rf rust/target

stable-%:
	sh -c "RUSTUP_TOOLCHAIN=stable cargo build --manifest-path rust/Cargo.toml --target $* --release"

so-apple: $(APPLE_TRIPLETS:%=stable-%)
so-android:
	cd rust && cargo ndk -t armeabi-v7a -t arm64-v8a -t x86_64 \
		build --release
