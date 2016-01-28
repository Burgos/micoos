#!/bin/sh

~/rust/arm-unknown-linux-gnueabi/bin/rustc -Z no-landing-pads --crate-type staticlib --target=arm-unknown-linux-gnueabi --emit obj -C linker=arm-linux-gnueabi-gcc  -C lto -C opt-level=3 $1
