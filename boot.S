.section ".text.boot"

.globl _start
	mov sp, #0x8000
	ldr r4, =__bss_start
	ldr r9, =__bss_end

	mov r5, #0
	mov r6, #0
	mov r7, #0
	mov r8, #0

1:
	stmia r4!, {r5-r8}

	cmp r4, r9
	blo 1b

	// call the kernel
	mov fp, #0
	ldr r3, =kernel
	blx r3

halt:
	wfe
	b halt

