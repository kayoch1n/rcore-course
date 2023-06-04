use alloc::vec::Vec;
use bitflags::bitflags;

use super::{
    address::{PageAddr, PhysPageNum, StepByOne, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
    VirtAddr,
};

bitflags! {
    pub struct PTEFlags: u8 {
        /// 是否已经被映射
        const V = 1 << 0;
        /// 可读
        const R = 1 << 1;
        /// 可写
        const W = 1 << 2;
        /// 可执行
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

/// 低 10bit 为flag
///
/// 44bit PPN
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self {
            bits: (ppn.0 << 10) | flags.bits as usize,
        }
    }

    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10).into()
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }

    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }

    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }

    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}

pub struct PageTable {
    /// 页表根节点
    root_ppn: PhysPageNum,
    /// 为什么要用一个列表存着？
    /// 貌似是为了切换程序的时候能自动释放
    frames: Vec<FrameTracker>,
}

impl PageTable {
    /// 创建一个根节点，同时会分配一个frame，用该frame ppn 拼接为 token
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();
        Self {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }

    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << PhysPageNum::WIDTH_SV39) - 1)),
            frames: Vec::new(),
        }
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find(vpn).map(|pte| pte.clone())
    }
    /// 将 vpn 和 ppn 的映射写入到页表
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte: &mut PageTableEntry = self.traverse(vpn).unwrap();
        assert!(!pte.is_valid(), "{:?} mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find(vpn).unwrap();
        assert!(pte.is_valid(), "{:?} is not mapped before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }
    /// 当PTE指向的page不存在的时候，会尝试分配一个frame
    fn traverse(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for i in 0..3 {
            let pte = &mut ppn.as_pte_array()[indexes[i]];
            if i == 2 {
                // 这里如果 as_pte_array 没有 static lifetime 的话
                // 就会出现编译错误，提示不能返回一个 borrow 了 local var 的 reference
                result = Some(pte);
                break;
            }
            // 如果这个 page table entry 不是有效的
            // 就分配一个 frame
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }

    fn find(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for i in 0..3 {
            let pte = &mut ppn.as_pte_array()[indexes[i]];
            if i == 2 {
                // 这里如果 as_pte_array 没有 static lifetime 的话
                // 就会出现编译错误，提示不能返回一个 borrow 了 local var 的 reference
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        result
    }
}

/// 返回一个 ptr 所在的物理地址的slice 的列表，这些位置有可能不是连续的
pub fn translate_byte_buffer(token: usize, ptr: *const u8, len: usize) -> Vec<&'static mut [u8]> {
    let page_table = PageTable::from_token(token);

    let mut start = ptr as usize;
    let end = start + len;
    let mut v = Vec::new();

    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();

        let ppn = page_table.find(vpn).unwrap().ppn();
        vpn.step();

        let mut end_va: VirtAddr = vpn.into();
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0 {
            v.push(&mut ppn.as_bytes()[start_va.page_offset()..])
        } else {
            v.push(&mut ppn.as_bytes()[start_va.page_offset()..end_va.page_offset()])
        }
        start = end_va.into();
    }
    v
}

pub fn copy_byte_buffer(src: &[u8], dest: Vec<&mut [u8]>) {
    let mut start = 0;
    for d in dest {
        d.copy_from_slice(&src[start..d.len()]);
        start += d.len();
    }
}
