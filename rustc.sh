#!/bin/sh
mkdir -p obj/
cargo rustc --features log-scheduler --target=arm-unknown-linux-gnueabi -- -Z no-landing-pads --crate-type staticlib  --emit obj -C linker=arm-linux-gnueabi-gcc  -C lto -C opt-level=3 -C relocation-model=dynamic-no-pic
