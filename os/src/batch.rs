use core::{
    arch::asm,
    mem,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use lazy_static::lazy_static;

use crate::{debug, info, sbi::shutdown, sync::UPSafeCell, trap::TrapContext};

const MAX_APP_NUM: usize = 8;
/// 对比：内核第一条指令的地址是 0x80200000
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x200000;

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    #[inline]
    fn stack_bottom(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> &mut TrapContext {
        /*
         + ---------------
         +  stack bottom
         + ---------------
         +  trap context
         + ---------------
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
            cx_ptr.as_mut().unwrap()
        }
    }
}

impl UserStack {
    #[inline]
    fn stack_bottom(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        info!("num_app = {}", self.num_app);
        for i in 0..self.num_app {
            info!(
                "app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
    /// 把应用程序从 bss 复制到 0x80400000
    ///
    /// PS: 内核的第一条指令从 0x80200000 开始
    unsafe fn load_app(&self, app_id: usize) {
        // Q:干嘛还要拷贝而不是直接跳转到这儿执行
        // A:因为user/bin下面的app都链接到同一个 base address 0x80400000
        // 不仅如此，装完之后还要告诉cpu要刷新 i-cache
        if app_id >= self.num_app {
            info!("All apps completed");
            shutdown();
        }
        debug!("Loading app_{}", app_id);

        from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);

        let app_src = from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );

        let app_dst = from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());

        app_dst.copy_from_slice(app_src);

        asm!("fence.i");
    }
}

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        // 把各个app的地址装进内存
        UPSafeCell::new({
            extern "C" {
                fn __num_app();
            }

            let start_ptr = __num_app as *const usize;
            let num_app = start_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];

            let app_start_ptr: &[usize] =
                core::slice::from_raw_parts(start_ptr.add(1), num_app + 1);

            app_start[..=num_app].copy_from_slice(app_start_ptr);

            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

pub fn run_next_app() -> ! {
    let mut app_manger = APP_MANAGER.exclusive_access();
    let current_app = app_manger.get_current_app();

    unsafe { app_manger.load_app(current_app) };
    app_manger.move_to_next_app();

    drop(app_manger);

    extern "C" {
        /// 会将参数 cx_addr 设置为 sp
        fn __restore(cx_addr: usize);
    }

    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::new(
            APP_BASE_ADDRESS,
            USER_STACK.stack_bottom(),
        )) as *const _ as usize)
    };

    panic!("unreachable!")
}

/// 打印各个app的信息。
///
/// 实际上 APP_MANAGER 是个静态变量已经通过全局初始化了
pub fn init() {
    debug!(
        "kernel stack\t[{:#x} ~ {:#x}]",
        KERNEL_STACK.data.as_ptr() as usize,
        KERNEL_STACK.stack_bottom()
    );
    debug!(
        "user stack\t[{:#x} ~ {:#x}]",
        USER_STACK.data.as_ptr() as usize,
        USER_STACK.stack_bottom()
    );

    APP_MANAGER.exclusive_access().print_app_info();
}
