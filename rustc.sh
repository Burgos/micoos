#!/bin/sh
mkdir -p obj/
~/rust/nightly/bin/rustc --out-dir=obj/ -Z no-landing-pads --crate-type staticlib --target=arm-unknown-linux-gnueabi --emit obj -C linker=arm-linux-gnueabi-gcc  -C lto -C opt-level=3 $1 -C relocation-model=dynamic-no-pic
