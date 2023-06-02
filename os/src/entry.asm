    .section .text.entry
    .globl _start
_start:
    # li x1, 100
    la sp, boot_stack_lower_bound
    call rust_main

    .section .bss.stack
    .globl boot_stack_top
boot_stack_top:
    .space 4096 * 16
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
# 栈从高地址向低地址生长
    