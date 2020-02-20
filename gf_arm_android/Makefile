#
# ```sh
# cat ~/.cargo/config
# ...
# [target.armv7-unknown-linux-gnueabihf]
# linker = "arm-linux-gnueabihf-gcc"
# 
# [target.aarch64-linux-android]
# linker = "/.../linux-x86_64/bin/aarch64-linux-android21-clang++"

cross:
	cargo build --target=aarch64-linux-android --release

b:
	cargo build

t:
	./gf_arm_android -g 222 --lock 1 2 3 --verbose
