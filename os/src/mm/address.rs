use core::{fmt::Debug, slice::from_raw_parts_mut};

use alloc::fmt;

use crate::config::{PAGEOFFSET_MASK, PAGESIZE, PAGESIZE_BITS, PAGETABLE_SIZE};

use super::page_table::PageTableEntry;

/// 物理地址
///
/// 0~11共12个bit表示offset，页大小为4KiB
///
/// 12~55表示PPN
#[derive(Clone, Copy)]
pub struct PhysAddr(pub usize);

/// 虚拟地址
///
/// 0~11共12个bit表示offset，页大小为4KiB
///
/// 12~38共27个bit表示VPN，最多128M个页
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub usize);

/// 从 0 开始计数的frame号码
///  
/// PPN 对应 frame
///
/// 不过 riscv64 手册里面就叫 PPN
#[derive(Clone, Copy)]
pub struct PhysPageNum(pub usize);

/// 从 0 开始计数的虚拟page号码
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct VirtPageNum(pub usize);

impl PhysAddr {
    const WIDTH_SV39: usize = 56;
}

impl VirtAddr {
    const WIDTH_SV39: usize = 39;
}

pub trait PageAddr<N>: Into<usize> + Copy
where
    N: From<usize>,
{
    // type T where Self::T: From<usize>;
    /// 返回 floor 处所在的 PageNum
    fn floor(&self) -> N {
        N::from(Into::<usize>::into(*self) / PAGESIZE)
    }

    fn ceil(&self) -> N {
        N::from((Into::<usize>::into(*self) + PAGESIZE - 1) / PAGESIZE)
    }

    /// page内的偏移
    fn page_offset(&self) -> usize {
        Into::<usize>::into(*self) & (PAGESIZE - 1)
    }

    fn get_mut(&self) -> *mut u8 {
        Into::<usize>::into(*self) as *mut u8
    }
}

impl PageAddr<PhysPageNum> for PhysAddr {}

impl PageAddr<VirtPageNum> for VirtAddr {}

impl PhysPageNum {
    pub const WIDTH_SV39: usize = PhysAddr::WIDTH_SV39 - PAGESIZE_BITS;
    /// 如果返回值没有 'static
    ///
    /// 编译器就会认为这个返回值是个局部变量
    pub fn as_bytes(&self) -> &'static mut [u8] {
        unsafe { from_raw_parts_mut(self.get_raw(), PAGESIZE) }
    }

    /// 返回一个指向当前 page num 对应的页表的指针
    fn get_raw(&self) -> *mut u8 {
        PhysAddr::from(self.clone()).get_mut()
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe { (self.get_raw() as *mut T).as_mut().unwrap() }
    }

    /// 返回当前 page num 对应的页表
    ///
    /// 其实就是 [PageTableEntry] 数组
    pub fn as_pte_array(&self) -> &'static mut [PageTableEntry] {
        unsafe { from_raw_parts_mut(self.get_raw() as *mut PageTableEntry, PAGETABLE_SIZE) }
    }
}

impl From<usize> for PhysAddr {
    fn from(value: usize) -> Self {
        Self(value & ((1 << Self::WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtAddr {
    fn from(value: usize) -> Self {
        Self(value & ((1 << Self::WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNum {
    /// 从一个 usize 中取出低 44 位作为 PPN
    fn from(value: usize) -> Self {
        Self(value & ((1 << PhysPageNum::WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtPageNum {
    /// 从一个 usize 中取出低 27 位作为 VPN
    fn from(value: usize) -> Self {
        VirtPageNum(value & ((1 << VirtPageNum::WIDTH_SV39) - 1))
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(value: PhysPageNum) -> Self {
        Self(value.0 << PAGESIZE_BITS)
    }
}

impl From<PhysPageNum> for usize {
    fn from(value: PhysPageNum) -> Self {
        value.0
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(value: PhysAddr) -> Self {
        assert_eq!(value.0 & PAGEOFFSET_MASK, 0);
        Self(value.0 / PAGESIZE)
    }
}

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("0x{:x}", self.0))
    }
}

impl fmt::Debug for PhysPageNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PhysPageNum")
            .field(&format_args!("{:?}", PhysAddr::from(*self)))
            .finish()
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("0x{:x}", self.0))
    }
}

impl fmt::Debug for VirtPageNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VirtPageNum")
            .field(&format_args!("{:?}", VirtAddr::from(*self)))
            .finish()
    }
}

impl From<PhysAddr> for usize {
    fn from(value: PhysAddr) -> Self {
        value.0
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(value: VirtPageNum) -> Self {
        Self(value.0 << PAGESIZE_BITS)
    }
}

impl From<VirtAddr> for usize {
    fn from(value: VirtAddr) -> Self {
        value.0
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(value: VirtAddr) -> Self {
        assert_eq!(value.0 & PAGEOFFSET_MASK, 0);
        Self(value.0 / PAGESIZE)
    }
}

impl VirtPageNum {
    const INDEX_WIDTH: usize = 9;
    const INDEX_MASK: usize = (1 << Self::INDEX_WIDTH) - 1;
    pub const WIDTH_SV39: usize = VirtAddr::WIDTH_SV39 - PAGESIZE_BITS;

    /// 返回一个数组，依次包括一级索引，二级索引和三级索引
    pub fn indexes(&self) -> [usize; 3] {
        let mut value = self.0;
        let mut ret = [0usize; 3];
        for i in (0..3).rev() {
            ret[i] = value & Self::INDEX_MASK;
            value >>= Self::INDEX_WIDTH;
        }
        ret
    }
}

pub trait StepByOne {
    fn step(&mut self);
}

impl StepByOne for VirtPageNum {
    fn step(&mut self) {
        self.0 += 1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimpleRange<T>
where
    T: PartialOrd + Debug + Copy,
{
    start: T,
    end: T,
}

impl<T> SimpleRange<T>
where
    T: PartialOrd + Debug + Copy,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start({:?}) > end({:?})", start, end);
        Self { start, end }
    }

    pub fn start(&self) -> T {
        self.start
    }

    pub fn end(&self) -> T {
        self.end
    }
}

impl<T> IntoIterator for SimpleRange<T>
where
    T: PartialOrd + Debug + Copy + StepByOne,
{
    type Item = T;

    type IntoIter = SimpleRangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator::new(self.start, self.end)
    }
}

pub struct SimpleRangeIterator<T> {
    current: T,
    end: T,
}

impl<T> SimpleRangeIterator<T> {
    pub fn new(current: T, end: T) -> Self {
        Self { current, end }
    }
}

impl<T> Iterator for SimpleRangeIterator<T>
where
    T: PartialEq + StepByOne + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}

pub type VirtPageRange = SimpleRange<VirtPageNum>;
