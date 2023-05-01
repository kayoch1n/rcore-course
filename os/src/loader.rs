use core::{
    arch::asm,
    mem,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::{
    config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT},
    config::{KERNEL_STACK_SIZE, MAX_APP_NUM, USER_STACK_SIZE},
    info,
    trap::TrapContext,
};

pub fn get_app_base(index: usize) -> usize {
    APP_BASE_ADDRESS + index * APP_SIZE_LIMIT
}

extern "C" {
    fn __num_app();
}

pub fn get_num_app() -> usize {
    unsafe { (__num_app as *const usize).read_volatile() }
}

/// 在内核栈底部创建app的初始trap context，写入入口地址和 app 的栈顶
///
/// 返回一个在内核栈的顶部的地址
///  
pub fn trap_init(app_id: usize) -> usize {
    KERNEL_STACKS[app_id].set_init_context(TrapContext::init(
        get_app_base(app_id),
        USER_STACKS[app_id].stack_bottom(),
    ))
}

pub fn init() -> usize {
    let num_app = get_num_app();

    let src_start = unsafe { from_raw_parts((__num_app as *const usize).add(1), num_app + 1) };

    unsafe {
        asm!("fence.i");
    }
    for app_id in 0..num_app {
        // 将app对应的内存的内容设置为0
        let dst_base = get_app_base(app_id);
        info!(
            "app_{} src: [{:#x}, {:#x}) dst: {:#x}",
            app_id,
            src_start[app_id],
            src_start[app_id + 1],
            dst_base,
        );
        (dst_base..dst_base + APP_SIZE_LIMIT)
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });

        unsafe {
            let src = from_raw_parts(
                src_start[app_id] as *const u8,
                src_start[app_id + 1] - src_start[app_id],
            );
            let dst = from_raw_parts_mut(dst_base as *mut u8, src.len());
            dst.copy_from_slice(src)
        }
    }

    num_app
}

#[derive(Clone, Copy)]
#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[derive(Clone, Copy)]
#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACKS: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

static USER_STACKS: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

impl KernelStack {
    #[inline]
    fn stack_bottom(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    /// 在内核栈的底部放入一个 trap context
    /// 并返回这个 trap context 在内核栈的地址
    pub fn set_init_context(&self, cx: TrapContext) -> usize {
        /*
         + ---------------
         +  stack bottom
         + ---------------
         +  trap context
         + --------------- <=== 返回这个
         +
         +      ...
         +
         +
         + ---------------
         +    stack top
         + ---------------
        */
        let cx_ptr = (self.stack_bottom() - mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        cx_ptr as usize
    }
}

impl UserStack {
    #[inline]
    fn stack_bottom(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
