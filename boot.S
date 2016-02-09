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
timer_0_intclr: .word 0x101E200C
irq_sp_save: .word 0x0
context_register_bank: .word 0x0

interrupt_routine:
    // http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0273a/Babcgdia.html
    stmfd r13!, {r0-r3, r12} // stack lr_irq and r12
    mrs r12, spsr
    stmfd r13!, {r12} // save spsr to stack

    // let's see if this is the timer?
    ldr r0, irq_status
    ldr r1, [r0]

    teq r1, #0x10
    bne exit_out

    // Load the interrupt address from PICVectAddr and execute it
    ldr     r0, irq_address
    ldr     r1, [r0]
    cmp r1, #0
    beq exit_out

    // it is the timer, clear the interrupt status
    ldr r2, timer_0_intclr
    mov r3, #0x1
    str r3, [r2]

    // interrupt service routine
    mov r0, r14
    blx     r1
    // new r14 is returned here
    mov r14, r0

    // unstack registers and lr_sys
    ldmfd r13!, {r0-r3}
    
    // -------- BACK TO INTERRUPT MODE ---------

exit_out:
    // unstack r12
    ldmfd sp!, {r12}
    // restore spsr
    msr spsr_cxsf, r12
    
    // ack. serving the interrupt
    ldr r1, irq_address
    mov r0, #0
    str r0, [r1]

    // unstack other registers
    ldmfd r13!, {r0-r3, r12}

    // return
    subs pc, lr, #4

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

.global asm_save_context
asm_save_context:
    ldr r1, irq_sp_save
    str r13, [r1]
    ldr r1, context_register_bank
    str r0, [r1]

    // load these from stack
    // TODO: jump spsr
    sub sp, sp, #4
    ldmfd r13!, {r0-r3}

    // change sp to point to context register bank
    ldr r13, =context_register_bank
    stmia r13, {r0-r14}^

    // restore irq stack pointer
    ldr r1, irq_sp_save
    ldr r13, [r1]
    bx lr

.section ".text.boot"

.globl _start
    // setup irq stack
    MSR     CPSR_c,   #0xd2 // Interrupts disabled irq mode
    mov     sp, #0x10000
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


