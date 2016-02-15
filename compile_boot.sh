arm-linux-gnueabi-gcc -fpie -ffreestanding -c boot.S -o boot.o -mcpu=arm1176jzf-s
arm-linux-gnueabi-gcc -fpie -ffreestanding -c arm.c -o aeabi.a -mcpu=arm1176jzf-s -std=c99
arm-linux-gnueabi-gcc -fpie -ffreestanding -c umodsi3.S -o umodsi3.o -mcpu=arm1176jzf-s
arm-linux-gnueabi-gcc -fpie -ffreestanding -c udivsi3.S -o udivsi3.o -mcpu=arm1176jzf-s
