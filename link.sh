#!/bin/sh

arm-linux-gnueabi-gcc -T arm.ld -nostartfiles -o kernel.elf -ffreestanding -nostdlib  memset.o  boot.o -lgcc lib.o

