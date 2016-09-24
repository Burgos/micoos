mkdir -p obj
arm-linux-gnueabi-gcc -fpie -ffreestanding -c src/asm/boot.S -o obj/boot.o -mcpu=arm1176jzf-s
arm-linux-gnueabi-gcc -fpie -ffreestanding -c src/runtime/arm.c -o obj/aeabi.a -mcpu=arm1176jzf-s -std=c99
arm-linux-gnueabi-gcc -fpie -ffreestanding -c src/runtime/gnu.c -o obj/gnu.a -mcpu=arm1176jzf-s -std=c99
arm-linux-gnueabi-gcc -fpie -ffreestanding -c src/runtime/umodsi3.S -o obj/umodsi3.o -mcpu=arm1176jzf-s
arm-linux-gnueabi-gcc -fpie -ffreestanding -c src/runtime/udivsi3.S -o obj/udivsi3.o -mcpu=arm1176jzf-s
