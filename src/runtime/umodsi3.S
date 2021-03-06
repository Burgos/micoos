/*===-- umodsi3.S - 32-bit unsigned integer modulus -----------------------===//
 *
 *                     The LLVM Compiler Infrastructure
 *
 * This file is dual licensed under the MIT and the University of Illinois Open
 * Source Licenses. See LICENSE.TXT for details.
 *
 *===----------------------------------------------------------------------===//
 *
 * This file implements the __umodsi3 (32-bit unsigned integer modulus) 
 * function for the ARM architecture.  A naive digit-by-digit computation is
 * employed for simplicity.
 *
 *===----------------------------------------------------------------------===*/

#include "assembly.h"

#define a r0
#define b r1
#define r r2
#define i r3

.syntax unified
.align 3
DEFINE_COMPILERRT_FUNCTION(__umodsi3)
//  We use a simple digit by digit algorithm; before we get into the actual 
//  divide loop, we must calculate the left-shift amount necessary to align
//  the MSB of the divisor with that of the dividend.
    clz     r2,     a
    tst     b,      b       // detect b == 0
    clz     r3,     b
    bxeq    lr              // return a if b == 0
    subs    i,      r3, r2
    bxlt    lr              // return a if MSB(a) < MSB(b)

LOCAL_LABEL(mainLoop):
//  This loop basically implements the following:
//
//  do {
//      if (a >= b << i) {
//          a -= b << i;
//          if (a == 0) break;
//      }
//  } while (--i)
//
//  Note that this does not perform the final iteration (i == 0); by doing it
//  this way, we can merge the two branches which is a substantial win for
//  such a tight loop on current ARM architectures.
    subs    r,      a,  b, lsl i
    movhs   a,      r
    subsne  i,      i, #1
    bhi     LOCAL_LABEL(mainLoop)

//  Do the final test subtraction and update of remainder (i == 0), as it is
//  not performed in the main loop.
    subs    r,      a,  b
    movhs   a,      r
    bx      lr

DEFINE_COMPILERRT_FUNCTION(__aeabi_ldivmod)
        push    {r11, lr}
        sub     sp, sp, #16
        add     r12, sp, #8
        str     r12, [sp]
        bl      SYMBOL_NAME(__divmoddi4)
        ldr     r2, [sp, #8]
        ldr     r3, [sp, #12]
        add     sp, sp, #16
        pop     {r11, pc}


DEFINE_COMPILERRT_FUNCTION(__aeabi_uidivmod)
    stmfd   sp!, {r0, r1, ip, lr}
    bl  __aeabi_uidiv
    ldmfd   sp!, {r1, r2, ip, lr}
    mul r3, r0, r2
    sub r1, r1, r3
    mov pc, lr


DEFINE_COMPILERRT_FUNCTION(__aeabi_uldivmod)
        push    {r11, lr}
        sub sp, sp, #16
        add r12, sp, #8
        str r12, [sp]
        bl  SYMBOL_NAME(__udivmoddi4)
        ldr r2, [sp, #8]
        ldr r3, [sp, #12]
        add sp, sp, #16
        pop {r11, pc}

