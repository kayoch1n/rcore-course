use core::slice::from_raw_parts;

use alloc::vec::Vec;
use lazy_static::lazy_static;

extern "C" {
    fn __num_app();
    fn __app_names();
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

pub fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
    let num = get_num_app();    
    (0..num).find(|&i| APP_NAMES[i] == name).map(|i| get_app_data(i))
}

lazy_static! {
    static ref APP_NAMES: Vec<&'static str> = {
        let num_app = get_num_app();
        let mut start = __app_names as *const u8;
        let mut v = Vec::new();

        unsafe {
            for _ in 0..num_app {
                let mut end = start;
                while end.read_volatile() != '\0' as u8 {
                    end = end.add(1)
                }
                let slice = core::slice::from_raw_parts(start, end as usize - start as usize);
                let s = core::str::from_utf8(slice).unwrap();
                v.push(s);
                start = end.add(1)
            }
        }
        v
    };
}
