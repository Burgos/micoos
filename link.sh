#!/bin/sh

arm-linux-gnueabi-gcc  -Wl,--build-id=none -T linker/arm.ld -nostartfiles -o kernel.elf -fpie -ffreestanding -nostdlib obj/boot.o obj/aeabi.a obj/umodsi3.o obj/udivsi3.o obj/lib.o

