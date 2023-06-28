.altmacro
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
    .global __restore
    .global __all_traps
    .align 2

__all_traps:
    # 为什么sscratch要指向一个 trap context 而不是 kernel stack
    # 因为要额外存储两个信息：kernel satp and trap handler
    # 只有一个 sscratch 根本不够用，所以把这几个值放到内存，

    # 这个 trap context 在 user space 中
    # before: sp 指向 user stack, sscratch 指向 TrapContext
    csrrw sp, sscratch, sp
    # now: sp 指向 TrapContext ，sscratch指向user stack
    sd x1, 1*8(sp)
    # 跳过 x2, x2 就是user sp
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
    # 
    # 上面这些代码都是在往 TRAP CONTEXT 写入内容，
    # 还没切换到 OS 栈
    # 
    ld sp, 34*8(sp)  # OS 栈的地址
    # sp+0 => x0, zero pointer
    # sp+1*8 => x1，用户ra
    # sp+2*8 => 用户stack，sp，也就是x2
    # sp+3*8 => x3
    # ...
    # sp+31*8 => x31
    # sp+32*8 => sstatus
    # sp+33*8 => sepc
    call trap_handler
__restore:
    # 第一个参数a0是 trap context
    csrw sscratch, a0

    mv sp, a0
    ld t1, 33*8(sp) # 在这个位置的是 返回地址 spec
    ld t0, 32*8(sp) # sstatus

    csrw sstatus, t0
    csrw sepc, t1

    ld x1, 1*8(sp)
    ld x3, 3*8(sp)

    .set n,5
    .rept 27
        LOAD_GP %n
        .set n,n+1
    .endr

    ld sp, 2*8(sp)
    sret