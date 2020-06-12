#!/bin/bash
set -e

cargo run --release -- compile ../docs/boot.s ../docs/boot.bin
xxd -p -c 4 ../docs/boot.bin | tail -n +2 > boot.rom
sbt "runMain build.Elaborate"