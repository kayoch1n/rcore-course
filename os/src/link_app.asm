
    .align 3
    .section .data
    .global __num_app
__num_app:
    .quad 2
    .quad __app_0_start
    .quad __app_1_start
    .quad __app_1_end

.global __app_names
__app_names:
    .string "initproc"
    .string "usrsh"

    .section .data

    .global __app_0_start
    .global __app_0_end
    .align 3
__app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/initproc"
__app_0_end:

    .global __app_1_start
    .global __app_1_end
    .align 3
__app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/usrsh"
__app_1_end:
