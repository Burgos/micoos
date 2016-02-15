#!/bin/sh

arm-linux-gnueabi-gcc  -Wl,--build-id=none -T arm.ld -nostartfiles -o kernel.elf -fpie -ffreestanding -nostdlib boot.o aeabi.a umodsi3.o udivsi3.o lib.o

