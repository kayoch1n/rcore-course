#![no_std]
#![no_main]

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {

    // write 报错 Illegal instruction
    // write_violate 报错 Illegal instruction
    unsafe { 
        core::ptr::null_mut::<u8>()
            .write_volatile(32) ;
    };

    0
}
/*
   0x000000008040003c <+0>:     addi    sp,sp,-16
   0x000000008040003e <+2>:     sd      ra,8(sp)
   0x0000000080400040 <+4>:     sd      s0,0(sp)
   0x0000000080400042 <+6>:     addi    s0,sp,16
=> 0x0000000080400044 <+8>:     unimp
 */


/*
   0x000000008040003c <+0>:     addi    sp,sp,-16
   0x000000008040003e <+2>:     sd      ra,8(sp)
   0x0000000080400040 <+4>:     sd      s0,0(sp)
   0x0000000080400042 <+6>:     addi    s0,sp,16
   0x0000000080400044 <+8>:     li      a0,32
   0x0000000080400048 <+12>:    sb      a0,0(zero) # 0x0
   0x000000008040004c <+16>:    li      a0,0
   0x000000008040004e <+18>:    ld      ra,8(sp)
   0x0000000080400050 <+20>:    ld      s0,0(sp)
   0x0000000080400052 <+22>:    addi    sp,sp,16
   0x0000000080400054 <+24>:    ret
 */