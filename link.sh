#!/bin/sh

arm-linux-gnueabi-gcc -T arm.ld -nostartfiles -o kernel.elf -ffreestanding -nostdlib boot.o lib.o

