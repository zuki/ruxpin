

all:
	ARCH=aarch64 cargo build --release
	rust-objcopy --strip-all -O binary target/aarch64-unknown-none/release/ruxpin ruxpin.img

dev:
	ARCH=aarch64 cargo build
	rust-objcopy -O binary target/aarch64-unknown-none/debug/ruxpin ruxpin.img
