arm-linux-gnueabi-gcc -fpie -ffreestanding -c boot.S -o boot.o -mcpu=arm1176jzf-s
arm-linux-gnueabi-gcc -fpie -ffreestanding -c arm.c -o aeabi.a -mcpu=arm1176jzf-s -std=c99
