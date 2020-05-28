#!/bin/sh

cargo run --release -- compile docs/boot.s docs/boot.bin
xxd -p -c 4 docs/boot.bin | tail -n +2 > rtl/boot.rom