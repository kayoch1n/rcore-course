.altmacro
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    
    .global __restore
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.align 2
__all_traps:
    csrrw sp, sscratch, sp # before: sp指向用户stack，sscratch指向内核stack
    # now: sp 指向内核stack，sscratch指向用户stack
    # 栈顶向下拉34*8个字节，也就是分配空间
    addi sp, sp, -34*8

    sd x1, 1*8(sp)
    # 跳过 x2
    sd x3, 3*8(sp)
    # 保存 x5~x31
    .set n,5
    .rept 27
        SAVE_GP %n
        .set n,n+1
    .endr


    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)  # 存 sstatus，trap之前的特权等级
    sd t1, 33*8(sp)  # 存 spec，trap之前的指令地址

    csrr t2, sscratch # 存用户stack
    sd t2, 2*8(sp)

    # a0作为trap_handler的第一个参数
    # sp+0 => x0, zero pointer
    # sp+1*8 => x1，用户ra
    # sp+2*8 => 用户stack，sp，也就是x2
    # sp+3*8 => x3
    # ...
    # sp+31*8 => x31
    # sp+32*8 => sstatus
    # sp+33*8 => sepc
    mv a0, sp
    call trap_handler
__restore:
    # sp 指向 内核 stack
    # 从内核栈上取出之前保存的寄存器的值
    ld t2, 2*8(sp) # 在这个位置的是 user stack
    ld t1, 33*8(sp) # 在这个位置的是 返回地址 spec
    ld t0, 32*8(sp) # sstatus

    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2 # user stack 放到 sscratch

    ld x1, 1*8(sp)
    ld x3, 3*8(sp)

    .set n,5
    .rept 27
        LOAD_GP %n
        .set n,n+1
    .endr

    addi sp, sp, 34*8
    csrrw sp, sscratch, sp # user stack 放到 sp，kernel stack 放到 sscratch
    sret