#!/bin/sh

arm-linux-gnueabi-gcc  -Wl,--build-id=none -T arm.ld -nostartfiles -o kernel.elf -fpie -ffreestanding -nostdlib boot.o lib.o

