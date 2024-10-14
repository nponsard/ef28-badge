# Embedded Rust on the Eurofurence 28 LED badge

This fork/branch contains the firmware I wrode in embedded rust for the EF28 badge.

## Setup

Install the esp-rs development environment (risc-v and Xtensa targets) [https://docs.esp-rs.org/book/](https://docs.esp-rs.org/book/).

Install the target for the risc-v coprocessor :

```sh
rustup target add riscv32imc-unknown-none-elf
rustup toolchain install nightly
```

Don't forget to import the esp-rs environement variables each time before opening the project.

## Building

First Build the coprocessor code :

```sh
cd coprocessor
cargo build --release
```

Then build the firmware :

```sh
cd main
cargo build --release
```

I noticed that if you only make changes to the coprocessor code and build it, the change won't be detected in the main firmware code and rust will not try to re-build the firmware. To force the inclusion of the new coprocessor code you can either clean the build files with `cargo clean` and rebuild (slower) or edit one line of code in the `main/src/main.rs` file to trigger a rebuild of a minimal portion of the project.  

## Flashing

You need to have at least built the coprocessor code first.

```sh
cd main
cargo run --release
```

This will build the firmware in release mode and upload it to the board connected via USB using `espflash`.
