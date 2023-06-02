OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x10000;

SECTIONS
{
    /* 需要指定一个基地址，这是因为目前rcore还不支持对使用相对地址的符号重定位 */
    /* 所以编译出来的程序有一些使用绝对地址的符号 */
    . = BASE_ADDRESS;

    /* 而且也没有栈 PS: 因为栈在OS那儿定义 */
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);

    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4K);

    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    sbss = .;
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    ebss = .;

    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}