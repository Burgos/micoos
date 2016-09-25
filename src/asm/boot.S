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
vital_instance: .word 0xdeadbeef
user_mode_pc: .word 0xdeadbeef
irq_sp: .word 0xdeadbeef

interrupt_routine:
    // http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0273a/Babcgdia.html
    stmfd r13!, {r0-r12} // stack lr_irq and r12

    // store previous user mode pc
    sub lr, lr, #4
    ldr r0, =user_mode_pc
    str lr, [r0]

    ldr r0, =irq_sp
    str r13, [r0]

    // let's see if this is the timer?
    ldr r0, irq_status
    ldr r1, [r0]

    teq r1, #0x10
    bne exit_out

    // Load the interrupt address from PICVectAddr and execute it
    ldr     r0, irq_address
    ldr     r4, [r0]
    cmp r4, #0
    beq exit_out

    // it is the timer, clear the interrupt status
    ldr r2, timer_0_intclr
    mov r3, #0x1
    str r3, [r2]

    // interrupt service routine
    ldr r2, =vital_instance
    ldr r0, [r2]
    mov r1, r14
    blx     r4

    // new r14 is returned here
    // load it from =user_mode_pc
    ldr r0, =user_mode_pc
    ldr lr, [r0]

    // -------- BACK TO INTERRUPT MODE ---------

exit_out:
    // ack. serving the interrupt
    ldr r1, irq_address
    mov r0, #0
    str r0, [r1]

    // unstack other registers
    ldmfd r13!, {r0-r12}

    // return
    subs pc, lr, #0

_reset:
    b .
__undef:
    b .

// Software interrupt handler
_svc:
    // load vital instance
    push {r1-r12, r14}
    mov r3, r2
    mov r2, r1
    mov r1, r0
    ldr r4, =vital_instance
    ldr r0, [r4]
    bl swi_interrupt_routine
    pop {r1-r12, r14}
    movs pc, r14


__prefetch_abort:
    b .
__data_abort:
    b .
_fiq:
    b .
_interrupt_routine:
    ldr pc, =interrupt_routine

.global asm_induce_swi
asm_induce_swi:
    swi 1
    bx lr


.global asm_save_context
asm_save_context:
    push {r0-r12}
    ldr r1, =irq_sp_save
    str r13, [r1]

    ldr r1, =context_register_bank
    str r0, [r1]

    // When we entered timer interrupt handler,
    // we saved r0-r12 and user mode pc on the
    // stack and in the memory mark. So we're going to find that mark
    // and read these from the stack. This is necessary
    // because in path irq_handler -> asm_save_context
    // we're going to pass to several check points,
    // corrupting general purpose registers.

    // we need  to pop r0-r12
    ldr r13, =irq_sp
    ldr r13, [r13]

    ldmfd r13!, {r0-r12}

    // now save them into the register bank
    ldr r13, =context_register_bank
    ldr r13, [r13]

    // push registers
    stmia r13, {r0-r14}^

    // push pc
    ldr r0, =user_mode_pc
    ldr r1, [r0]
    add r13, r13, #60 // 4 * 15 registers
    stmia r13, {r1}

    // push spsr
    add r13, r13, #4
    mrs r0, spsr
    stmia r13, {r0}

    // restore irq stack pointer
    ldr r1, =irq_sp_save
    ldr r13, [r1]

    nop
    pop {r0-r12}
    bx lr

.global asm_restore_context
asm_restore_context:
    push {r0-r12}
    ldr r1, =irq_sp_save
    str r13, [r1]

    nop

    ldr r1, =context_register_bank
    str r0, [r1]

    // we need to pop r0-r5
    ldr r13, =irq_sp
    ldr r13, [r13]

    ldmfd r13!, {r0-r12}

    // now save them into the register bank
    ldr r13, =context_register_bank
    ldr r13, [r13]

    // pop pc
    add r13, r13, #60
    ldmia r13, {r1}
    ldr r0, =user_mode_pc
    str r1, [r0]
    
    // pop spsr
    add r13, r13, #4
    ldmia r13, {r0}
    msr spsr, r0
    
    sub r13, r13, #64
    // pop other registers
    ldmia r13, {r0-r14}^

    nop

    /*
        save them on the stack of the irq
        At this point, stack at the entering of the irq looks like

        r0
        r1
        r2
        ...
        r12   <- irq_sp points here

        We need to move back to r0 - increasing sp
        with 13*4 bytes and then replacing all these
        with the ones from the process.

    */

    ldr r13, =irq_sp
    ldr r13, [r13]
    add r13, r13, #52 // 13 registers * 4 bytes

    // save them to irq_sp stack
    stmfd r13!, {r0-r12} // stack lr_irq and r12

    // restore irq stack pointer
    ldr r1, =irq_sp_save
    ldr r13, [r1]

    //b .
    nop
    pop {r0-r12}
    bx lr

.global asm_set_vital_instance
asm_set_vital_instance:
    ldr r1, =vital_instance
    str r0, [r1]
    blx lr

.section ".text.boot"

.globl _start
    // setup irq stack
    MSR     CPSR_c,   #0xd2 // Interrupts disabled irq mode
    mov     sp, #0x20000
    MSR     CPSR_c, #0xd3 // svc mode
    mov     sp, #0x22000
    MSR     CPSR_c,  #0xdf // Interrupts disabled system mode

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


	// call the kernel
	mov fp, #0
	ldr r3, =kernel
	blx r3

.global wait_for_event
wait_for_event:
	wfe
	b wait_for_event


vector_start:
    ldr pc, =interrupt_routine


