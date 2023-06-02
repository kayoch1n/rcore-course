use crate::{
    debug,
    mm::address::{PageAddr, PhysAddr},
};
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

use super::address::PhysPageNum;

pub const MEMORY_END: usize = 0x80800000;

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
    /// 下一个要分配的 PPN
    next: usize,
    /// 最后一个要分配的 PPN，不包括
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    /// 由于用到了 vec，所以需要在堆初始化之后执行
    fn init(&mut self, start: PhysPageNum, end: PhysPageNum) {
        self.next = start.0;
        self.end = end.0;
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        StackFrameAllocator {
            next: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }
    /// 分配一个frame，其实只是一个数字而已
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else {
            if self.next == self.end {
                None
            } else {
                self.next += 1;
                Some((self.next - 1).into())
            }
        }
    }
    /// 回收一个 frame，其实只是把这个page num
    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if ppn >= self.next || self.recycled.iter().find(|&v| *v == ppn).is_some() {
            panic!("PPN={:#x} has not been allocated!", ppn);
        }

        self.recycled.push(ppn)
    }
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<StackFrameAllocator> =
        Mutex::new(StackFrameAllocator::new());
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }

    let start = PhysAddr::from(ekernel as usize).ceil();
    let end = PhysAddr::from(MEMORY_END).floor();
    let mut allocator = FRAME_ALLOCATOR.lock();
    allocator.init(start, end);
    debug!(
        "Frames\t[{:?} ~ {:?})",
        Into::<PhysAddr>::into(start),
        Into::<PhysAddr>::into(end)
    );
    debug!("frame allocator: next=0x{:x}, end=0x{:x}", allocator.next, allocator.end);
}

/// 回收一个frame
pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.lock().dealloc(ppn)
}

/// 分配一个frame ，其实就是从OS镜像的末尾开始的物理地址以后的那一部分来的
pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.lock().alloc().map(FrameTracker::new)
}

/// [FrameTracker] 被 drop 之后，frame会被归还到 [FRAME_ALLOCATOR]
#[derive(Debug)]
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    fn new(ppn: PhysPageNum) -> Self {
        for i in ppn.as_bytes() {
            *i = 0
        }
        Self { ppn }
    }
}

// TODO: 什么时候会被 drop 掉
impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn)
    }
}
