OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry);
        . = ALIGN(4K);

        /* 
            跳板代码在单独一个frame中，然后os把os和user的跳板page
            设置成一样的虚拟地址，同时都给映射到这个frame */
        strampoline = .;
        *(.text.trampoline);
        . = ALIGN(4K);

        *(.text .text.*)
    }
    . = ALIGN(4K);
    etext = .;

    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        /* What is srodata ? */
        /* https://www.reddit.com/r/gcc/comments/pgjhey/comment/hbcha7c/?utm_source=share&utm_medium=web2x&context=3 */
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    erodata = .;

    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    edata = .;

    sbss = .;
    .bss : {
        *(.bss.stack)
        sbss_after_stack = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}