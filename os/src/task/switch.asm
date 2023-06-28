.altmacro
.macro SAVE_SN n # 将n个寄存器保存到从a0+8*2开始的位置
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n # 从 a1+8*2开始的位置读取数据到n个寄存器
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .global __switch
__switch:
    # 函数签名 __switch(current_task_ptr, next_task_ptr, token)
    # a0 a1 a2 分别对应三个参数
    # 这三个参数所代表的内存地址必须在所有task中都有映射！
    # 保存各个s0~s11寄存器(共12个)，还有sp和ra

    # 切换satp的过程从 __restore 改到 __switch
    csrw satp, a2
    sfence.vma

    sd sp, 8(a0)
    sd ra, 0(a0)
    .set n,0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
    # a1 是第二个参数
    .set n,0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    ld sp,8(a1)
    ld ra,0(a1)
    ret
