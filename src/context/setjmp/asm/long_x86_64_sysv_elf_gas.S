/*
            Copyright Oliver Kowalke 2009.
   Distributed under the Boost Software License, Version 1.0.
      (See accompanying file LICENSE_1_0.txt or copy at
            http://www.boost.org/LICENSE_1_0.txt)
*/

/****************************************************************************************
 *                                                                                      *
 *  ----------------------------------------------------------------------------------  *
 *  |    0    |    1    |    2    |    3    |    4     |    5    |    6    |    7    |  *
 *  ----------------------------------------------------------------------------------  *
 *  |   0x0   |   0x4   |   0x8   |   0xc   |   0x10   |   0x14  |   0x18  |   0x1c  |  *
 *  ----------------------------------------------------------------------------------  *
 *  | fc_mxcsr|fc_x87_cw|        R12        |         R13        |        R14        |  *
 *  ----------------------------------------------------------------------------------  *
 *  ----------------------------------------------------------------------------------  *
 *  |    8    |    9    |   10    |   11    |    12    |    13   |    14   |    15   |  *
 *  ----------------------------------------------------------------------------------  *
 *  |   0x20  |   0x24  |   0x28  |  0x2c   |   0x30   |   0x34  |   0x38  |   0x3c  |  *
 *  ----------------------------------------------------------------------------------  *
 *  |        R15        |        RBX        |         RBP        |        RIP        |  *
 *  ----------------------------------------------------------------------------------  *
 *                                                                                      *
 ****************************************************************************************/

.file "long_x86_64_sysv_elf_gas.S"
.text
.globl longjmp
.type longjmp,@function
.align 16
longjmp:
    movq 8(%rdi), %rbx  # Restore %rbx
    movq 16(%rdi), %rsp # Restore %rsp
    movq 24(%rdi), %rbp # Restore %rbp
    movq 32(%rdi), %r12 # Restore %r12
    movq 40(%rdi), %r13 # Restore %r13
    movq 48(%rdi), %r14 # Restore %r14
    movq 56(%rdi), %r15 # Restore %r14
    movq (%rdi), %rax   # Move new %rip in stack
    movq %rax, (%rsp)
    movq %rsi, %rax     # Set return value
    ret
.size longjmp,.-longjmp

/* Mark that we don't need executable stack.  */
.section .note.GNU-stack,"",%progbits
