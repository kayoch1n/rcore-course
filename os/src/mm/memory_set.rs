use alloc::{collections::BTreeMap, vec::Vec};
use bitflags::bitflags;

use crate::{
    config::{KERNEL_BASE, PAGESIZE, TRAP_CONTEXT, TRAP_CONTEXT_END, USER_STACK_SIZE},
    debug, ebss, edata, ekernel, erodata, etext,
    mm::address::StepByOne,
    sbss, sdata, srodata, stext,
};

use super::{
    address::{PageAddr, PhysPageNum, VirtAddr, VirtPageNum, VirtPageRange},
    frame_allocator::{frame_alloc, FrameTracker, MEMORY_END},
    page_table::{PTEFlags, PageTable},
};

#[derive(Debug, PartialEq, Eq)]
pub enum MapType {
    /// virtual address 和 physical address 一致
    Identical,
    /// 随机映射
    Framed,
}

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

/// 其实就是 segment
pub struct Segment {
    /// 映射方式
    map_type: MapType,
    /// 权限
    map_perm: MapPermission,
    /// 连续的若干个页
    vpn_range: VirtPageRange,
    /// page和对应的frame
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
}

impl Segment {
    pub fn map(&mut self, page_table: &mut PageTable) -> usize {
        let mut count = 0;
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
            count += 1
        }
        count
    }

    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(page_table, vpn)
        }
    }

    fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => ppn = PhysPageNum(vpn.0),
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }
        let flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        page_table.map(vpn, ppn, flags)
    }

    fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_type == MapType::Framed {
            self.data_frames.remove(&vpn);
        }
        page_table.unmap(vpn)
    }

    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);

        let mut start: usize = 0;
        // TODO: 改成 for in ？
        let mut current_vpn = self.vpn_range.start();

        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGESIZE)];
            // 到这里代码还是访问的物理内存
            let dst = &mut page_table.translate(current_vpn).unwrap().ppn().as_bytes()[..src.len()];

            dst.copy_from_slice(src);
            start += PAGESIZE;
            if start >= len {
                break;
            }
            current_vpn.step()
        }
    }
    /// 创建一个虚拟地址从 start 到 end 的 segment 结构
    pub fn new(start: VirtAddr, end: VirtAddr, map_type: MapType, map_perm: MapPermission) -> Self {
        Self {
            map_type,
            map_perm,
            vpn_range: VirtPageRange::new(start.floor(), end.ceil()),
            data_frames: BTreeMap::new(),
        }
    }
}

/// 一个进程的地址空间
pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<Segment>,
}

impl MemorySet {
    /// 创建一个地址空间，配套有一个新页表
    fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    fn push(&mut self, mut seg: Segment, data: Option<&[u8]>) -> usize {
        let count = seg.map(&mut self.page_table);
        if let Some(data) = data {
            seg.copy_data(&mut self.page_table, data)
        }
        self.areas.push(seg);
        count
    }

    pub fn insert_segment(&mut self, start: VirtAddr, end: VirtAddr, permission: MapPermission) {
        self.push(Segment::new(start, end, MapType::Framed, permission), None);
    }

    /// 用 identical 映射
    pub fn map_kernel(&mut self) -> usize {
        // OS的 text、data、rodata、bss全部使用identical mapping
        let mut page_count = 0;
        page_count += self.push(
            Segment::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );

        page_count += self.push(
            Segment::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
            ),
            None,
        );

        page_count += self.push(
            Segment::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        page_count += self.push(
            Segment::new(
                (sbss as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        // 后面剩余的所有内存，包括没分配的frame，都用 identical 映射
        page_count += self.push(
            Segment::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        page_count
    }

    /// 创建一个 elf 的地址空间
    /// 将 elf 的各个 segment 和栈的虚拟地址都映射到物理内存
    /// 返回地址空间对象，栈底和入口地址
    pub fn new_elf(data: &[u8]) -> (Self, usize, usize) {
        let mut ms = Self::new_bare();
        let mut page_count = 0;

        let elf = xmas_elf::ElfFile::new(data).unwrap();
        let elf_header = elf.header;
        assert_eq!(
            elf_header.pt1.magic,
            [0x7f, 0x45, 0x4c, 0x46],
            "invalid elf!"
        );

        // let ph_count = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);
        // 将ELF中的各个段映射到内存
        for i in 0..elf_header.pt2.ph_count() {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start: VirtAddr = (ph.virtual_addr() as usize).into();
                let end: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();

                let mut map_perm = MapPermission::U;
                let flags = ph.flags();
                if flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                let seg = Segment::new(start, end, MapType::Framed, map_perm);
                max_end_vpn = seg.vpn_range.end();
                page_count += ms.push(
                    seg,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
            }
        }

        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_top: usize = max_end_va.into();
        // guard page
        user_stack_top += PAGESIZE;
        // user stack 在所有段的最后
        let user_stack_bottom = user_stack_top + USER_STACK_SIZE;
        page_count += ms.push(
            Segment::new(
                user_stack_top.into(),
                user_stack_bottom.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::U | MapPermission::W,
            ),
            None,
        );
        assert!(
            user_stack_bottom <= KERNEL_BASE,
            "user stack(0x{:x}) cannot be greater than kernel base(0x{:x})",
            user_stack_bottom,
            KERNEL_BASE
        );
        // 映射 OS
        page_count += ms.map_kernel();
        // 最后是trap context
        page_count += ms.push(
            Segment::new(
                TRAP_CONTEXT.into(),
                TRAP_CONTEXT_END.into(),
                MapType::Framed,
                MapPermission::W | MapPermission::R,
            ),
            None,
        );
        debug!(
            "{} page(s) used; user stack bottom: 0x{:x}",
            page_count, user_stack_bottom
        );
        (ms, user_stack_bottom, elf.header.pt2.entry_point() as usize)
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<super::page_table::PageTableEntry> {
        self.page_table.translate(vpn)
    }

    pub fn token(&self) -> usize {
        self.page_table.token()
    }
}
