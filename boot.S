.section ".rodata"

.globl __reset
__reset:

   ldr   pc, reset_handler_address
   ldr   pc, undef_handler_address
   ldr   pc, svc_handler_address
   ldr   pc, prefetch_abort_handler_address
   ldr   pc, data_abort_handler_address
_loop:   b   .
   ldr   pc, =interrupt_routine
   ldr   pc, fiq_handler_address

reset_handler_address:      .word   _reset
undef_handler_address:      .word   __undef
svc_handler_address:      .word   _svc
prefetch_abort_handler_address:   .word   __prefetch_abort
data_abort_handler_address:   .word   __data_abort
unused:            .word   _loop
irq_handler_address:      .word   _interrupt_routine
fiq_handler_address:      .word   _fiq

irq_address:    .word 0x10140030
irq_status:     .word 0x10140000


interrupt_routine:
    SUB     lr, lr, #4             //. construct the return address
    PUSH    {lr}                   //. and push the adjusted lr_IRQ
    MRS     lr, SPSR               //. copy spsr_IRQ to lr
    PUSH    {R0-R4,R12,lr}         //. save AAPCS regs and spsr_IRQ
    MSR     CPSR_c, #0x9F          //. switch to SYS mode, IRQ is
                                   //. still disabled. USR mode
                                   //. registers are now current.
    #AND     R1, sp, #4             //. test alignment of the stack
    #SUB     sp, sp, R1             //. remove any misalignment (0 or 4)
    #PUSH    {R1,lr}                //. store the adjustment and lr_USR
    #MSR     CPSR_c, #0x1F          //. enable IRQ

    // load the status and check for the interrupt
    ldr r0, irq_status
    ldr r1, [r0]
    beq exit_out

    // Load the interrupt address from PICVectAddr and execute it
    ldr     r0, irq_address
    ldr     r1, [r0]
    beq exit_out
    blx     r1

exit_out:
    MSR     CPSR_c, #0x9F          //. disable IRQ, remain in SYS mode
    #POP     {R1,lr}                //. restore stack adjustment and lr_USR
    #ADD     sp, sp, R1             //. add the stack adjustment (0 or 4)
    MSR     CPSR_c, #0x92          //. switch to IRQ mode and keep IRQ
                                   //. disabled. FIQ is still enabled.
    POP     {R0-R4,R12,lr}         //. restore registers and
    MSR     SPSR_cxsf, lr          //. spsr_IRQ
    LDM     sp!, {pc}^             //. return from IRQ.

_reset:
    b .
__undef:
    b .
_svc:
    b .
__prefetch_abort:
    b .
__data_abort:
    b .
_fiq:
    b .
_interrupt_routine:
    ldr pc, =interrupt_routine

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


    #mov r0, #0x18
    #ldr r1, vector_start
    #str r1, [r0]

	// call the kernel
	mov fp, #0
	ldr r3, =kernel
	blx r3

halt:
	wfe
	b halt


vector_start:
    ldr pc, =interrupt_routine


