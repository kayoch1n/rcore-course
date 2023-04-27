use core::mem;

use lazy_static::lazy_static;

use crate::{
    debug, info,
    loader::{get_app_base, load_apps},
    sbi::shutdown,
    sync::UPSafeCell,
    trap::TrapContext,
};

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
}

impl AppManager {
    pub fn move_to_next_app(&mut self) {
        self.current_app += 1
    }
}

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe { UPSafeCell::new(AppManager { num_app: 0, current_app: 0 }) };
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    extern "C" {
        /// 会将参数 cx_addr 设置为 sp
        fn __restore(cx_addr: usize);
    }
    let next_app = app_manager.current_app;
    let total_apps = app_manager.num_app;
    
    if next_app + 1 > total_apps {
        drop(app_manager);
        info!("all apps finished");
        shutdown()
    } else {
        app_manager.move_to_next_app();
        drop(app_manager);
    }

    let entry = get_app_base(next_app);
    debug!("run app_{} at {:#x}", next_app, entry);

    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::new(
            entry,
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
    APP_MANAGER.exclusive_access().num_app = load_apps();
}
