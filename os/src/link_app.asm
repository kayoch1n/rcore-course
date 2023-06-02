
    .align 3
    .section .data
    .global __num_app
__num_app:
    .quad 7
    .quad __app_0_start
    .quad __app_1_start
    .quad __app_2_start
    .quad __app_3_start
    .quad __app_4_start
    .quad __app_5_start
    .quad __app_6_start
    .quad __app_6_end

    .section .data

    .global __app_0_start
    .global __app_0_end
    .align 3
__app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00hello_world"
__app_0_end:

    .global __app_1_start
    .global __app_1_end
    .align 3
__app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/01store_fault"
__app_1_end:

    .global __app_2_start
    .global __app_2_end
    .align 3
__app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/02priv_inst"
__app_2_end:

    .global __app_3_start
    .global __app_3_end
    .align 3
__app_3_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/03priv_csr"
__app_3_end:

    .global __app_4_start
    .global __app_4_end
    .align 3
__app_4_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/04float"
__app_4_end:

    .global __app_5_start
    .global __app_5_end
    .align 3
__app_5_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/05power_3"
__app_5_end:

    .global __app_6_start
    .global __app_6_end
    .align 3
__app_6_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/06power_7"
__app_6_end:
