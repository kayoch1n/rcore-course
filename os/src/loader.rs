use core::{
    arch::asm,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::{
    batch::{APP_BASE_ADDRESS, APP_SIZE_LIMIT},
    info,
};

pub fn get_app_base(index: usize) -> usize {
    APP_BASE_ADDRESS + index * APP_SIZE_LIMIT
}

pub fn load_apps() -> usize {
    extern "C" {
        fn __num_app();
    }

    let num_app_ptr = __num_app as *const usize;
    let num_app = unsafe { num_app_ptr.read_volatile() };

    let src_start = unsafe { from_raw_parts(num_app_ptr.add(1), num_app + 1) };

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
