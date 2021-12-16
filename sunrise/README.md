+ [rust-cross-compile-example](https://github.com/nicolas-van/rust-cross-compile-example)

```sh
yay -Ss aarch64-linux-gnu|rg community|less
community/aarch64-linux-gnu-linux-api-headers 5.8-1 (1.1 MiB 5.0 MiB) (установлено)
community/aarch64-linux-gnu-glibc 2.34-1 (3.4 MiB 19.3 MiB) (установлено)
community/aarch64-linux-gnu-gdb 11.1-2 (3.1 MiB 9.1 MiB) (установлено)
community/aarch64-linux-gnu-gcc 11.2.0-1 (43.9 MiB 267.6 MiB) (установлено)
community/aarch64-linux-gnu-binutils 2.37-1 (3.7 MiB 30.7 MiB) (установлено)
#yay -S aur/aarch64-elf-gcc-linaro-bin
#yay -S aur/aarch64-elf-newlib-linaro-bin 
```

```sh
cat ~/.cargo/config
```

``` ~/.cargo/config
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

[target.aarch64-linux-android]
linker = "/opt/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang++"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

```
rustup target add aarch64-unknown-linux-gnu
cargo build --release --verbose --target aarch64-unknown-linux-gnu
```
