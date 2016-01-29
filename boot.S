.section ".text.boot"

.globl _start
    // setup irq stack
    MSR     CPSR_c,   #0xd2 // Interrupts disabled irq mode
    MOV     sp, R0
    SUB     R0, R0, #0x10000
    MSR     CPSR_c,  #0xdF // Interrupts disabled svc mode

    // svc mode

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

    // setup the irq
    ldr r0, =timer0_iv
    ldr r1, =interrupt_routine
    str r1, [r0]

	// call the kernel
	mov fp, #0
	ldr r3, =kernel
	blx r3

.globl interrupt_routine
    SUB     lr, lr, #4             //. construct the return address
    PUSH    {lr}                   //. and push the adjusted lr_IRQ
    MRS     lr, SPSR               //. copy spsr_IRQ to lr
    PUSH    {R0-R4,R12,lr}         //. save AAPCS regs and spsr_IRQ
    MSR     CPSR_c, #0x9F          //. switch to SYS mode, IRQ is
                                   //. still disabled. USR mode
                                   //. registers are now current.
    AND     R1, sp, #4             //. test alignment of the stack
    SUB     sp, sp, R1             //. remove any misalignment (0 or 4)
    PUSH    {R1,lr}                //. store the adjustment and lr_USR
    MSR     CPSR_c, #0x1F          //. enable IRQ
    ldr     r3,=timer_interrupt_routine
    BLX     r3
    MSR     CPSR_c, #0x9F          //. disable IRQ, remain in SYS mode
    //POP     {R1,lr}                //. restore stack adjustment and lr_USR
    //ADD     sp, sp, R1             //. add the stack adjustment (0 or 4)
    MSR     CPSR_c, #0x92          //. switch to IRQ mode and keep IRQ
                                   //. disabled. FIQ is still enabled.
    POP     {R0-R4,R12,lr}         //. restore registers and
    MSR     SPSR_cxsf, lr          //. spsr_IRQ
    LDM     sp!, {pc}^             //. return from IRQ.

halt:
	wfe
	b halt

