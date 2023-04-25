
    .align 3
    .section .data
    .global __num_app
    .global __end_app
__num_app:
    .quad 4
.quad app_0_start
.quad app_1_start
.quad app_2_start
.quad app_3_start

    .section .data

    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00hello_world.bin"
app_0_end:

    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/01store_fault.bin"
app_1_end:

    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/02priv_inst.bin"
app_2_end:

    .global app_3_start
    .global app_3_end
app_3_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/03priv_csr.bin"
app_3_end:

__end_app:
    .quad 0
    
