#!/bin/sh
RUSTFLAGS="-C code-model=medium -C relocation-model=static -C link-arg=-Tmemory.x -C link-arg=-Tmylink.x" cargo +nightly run -Z build-std --target riscv32imac-unknown-none-elf
