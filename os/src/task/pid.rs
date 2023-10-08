use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct PidHandle(pub usize);

struct PidAllocator {
    current: usize,
    recycled: Vec<usize>
}

impl PidAllocator {
    pub fn new() -> Self {
        PidAllocator { current: 0, recycled: Vec::new() }
    }

    pub fn alloc(&mut self) -> PidHandle {
        if let Some(old) = self.recycled.pop() {
            PidHandle(old)
        } else {
            let pid = self.current;
            self.current += 1;
            PidHandle(pid)
        }
    }

    pub fn dealloc(&mut self, pid: usize) {
        assert!(pid < self.current);
        assert!(self.recycled.iter().find(|&&old| old == pid).is_none(), "pid {} has been deallocated!", pid);
        self.recycled.push(pid);
    }
}

lazy_static! {
    static ref PID_ALLOCATOR: Mutex<PidAllocator> = Mutex::new(PidAllocator::new());
}

pub fn pid_alloc() -> PidHandle {
    PID_ALLOCATOR.lock().alloc()
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().dealloc(self.0)
    }
}