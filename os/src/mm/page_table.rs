use super::{frame_alloc, FrameTracker, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
use alloc::vec;
use alloc::vec::Vec;
use bitflags::*;

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

/// 页表项结构体 (PTE, Page Table Entry)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

// 页表项相关方法
impl PageTableEntry {

    // 创建一个页表项
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }
    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}

// 页表结构体，就是一个应用拥有的页表项PTE集合
pub struct PageTable {
    root_ppn: PhysPageNum,
    // 加入frames这一项可以以页表为单位对页表项集中管理
    frames: Vec<FrameTracker>,
}

/// Assume that it won't oom when creating/mapping.
impl PageTable {

    // 页表的创建就是创建根页表项,同时把页表项加入向量中
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();
        PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    // 临时创建一个专用来手动查页表的 PageTable ，它仅有一个从传入的 satp token 中得到的多级页表根节点的物理页号，它的 frames 字段为空，也即不实际控制任何资源；
    /// Temporarily used to get arguments from user space.
    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
        }
    }

    // 根据虚拟页号找第三级物理页帧，没有就创建
    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }

    // 根据虚拟页号找第三级物理页帧
    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;

        // i 和 idx 是同时递增的
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
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

    // 创建虚拟地址到物理地址的映射时使用，参数ppn是由frame_allocator分配来的，是第三级页表上的物理页号中要填写的地址，也就是应用查找的实际物理地址
    #[allow(unused)]
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);

        // 在第三级页表上填写物理地址相应的信息
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    #[allow(unused)]
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }

    // 根据虚拟地址找第三级页表上相应的页表项
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }

    // 按satp格式要求构造64位数
    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }

    // 根据虚拟地址找第三级页表上相应的页表项, 没有就创建
    pub fn translate_create(&mut self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte_create(vpn).map(|pte| *pte)
    }
}

pub fn translated_byte_buffer(token: usize, ptr: *const u8, len: usize) -> Vec<&'static mut [u8]> {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut v = Vec::new();
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppn = page_table.translate(vpn).unwrap().ppn();
        vpn.step();
        let mut end_va: VirtAddr = vpn.into();
        // end_va 和 end 比谁小
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0 {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..]);
        } else {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
        }
        start = end_va.into();
    }
    v
}

// 分配页表，成功返回实际分配的物理页，失败返回-1
pub fn alloc_pages(token: usize, start: usize, len: usize, port: usize) -> isize{
    let mut page_table = PageTable::from_token(token);
    println!("start:0x{:X}", start);
    println!("port:{:b}", port);
    let end = start + len;
    println!("end:0x{:X}", end);
    let start_va = VirtAddr::from(start);
    println!("start_va:{:?}", start_va);
    let start_vpn = start_va.floor();
    println!("start_vpn:{:?}", start_vpn);
    let end_va = VirtAddr::from(end);
    println!("end_va:{:?}", end_va);
    let end_vpn = end_va.ceil();
    println!("end_vpn:{:?}", end_vpn);
    let mut vpn = start_vpn;
    println!("vpn:{:?}", vpn);
    while vpn < end_vpn {
        let pte = page_table.translate_create(vpn).unwrap();
        if pte.is_valid(){
            return -1;
        } else {
            let frame = frame_alloc().unwrap();
            let ppn = frame.ppn;
            println!("ppn:{:?}", ppn);
            let flags = PTEFlags{bits: ((port as u8) << 1)};
            page_table.map(vpn, ppn, flags);
        }
        vpn = VirtPageNum::from(usize::from(vpn) + 1);
    }
    0
}
