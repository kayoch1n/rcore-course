use core::slice::from_raw_parts;

extern "C" {
    fn __num_app();
}

pub fn get_num_app() -> usize {
    unsafe { (__num_app as *const usize).read_volatile() }
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    let num_app_ptr = __num_app as *const usize;
    let num_app = get_num_app();

    assert!(app_id < num_app);
    let app_start = unsafe { from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    unsafe {
        from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}
