#!/bin/sh

arm-linux-gnueabi-gcc  -Wl,--build-id=none -T arm.ld -nostartfiles -o kernel.elf -ffreestanding -nostdlib boot.o lib.o

