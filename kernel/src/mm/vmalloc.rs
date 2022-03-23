
use alloc::vec::Vec;

use crate::mm::pages;
use crate::mm::MemoryAccess;
use crate::arch::mmu::{self, TranslationTable, VirtualAddress, PhysicalAddress};


const MAX_SEGMENTS: usize = 6;


pub fn init_virtual_memory(start: *mut u8, end: *mut u8) {
    pages::init_pages_area(start, end);
}

pub struct Segment {
    start: VirtualAddress,
    end: VirtualAddress,
    //ops for getting pages
}

pub struct VirtualAddressSpace {
    table: TranslationTable,
    segments: Vec<Segment>,
}

impl VirtualAddressSpace {
    pub fn new_user_space() -> Self {
        let pages = pages::get_page_area();
        let table = TranslationTable::new_user_table(pages);

        Self {
            table,
            segments: Vec::with_capacity(MAX_SEGMENTS),
        }
    }

    pub fn alloc_mapped(&mut self, access: MemoryAccess, mut vaddr: VirtualAddress, length: usize) -> *mut u8 {
        let pages = pages::get_page_area();
        // TODO this needs to be replaced when then page allocator can do blocks
        let mut first = 0;
        for _ in 0..(length / mmu::page_size()) {
            let ptr = pages.alloc_page_zeroed() as PhysicalAddress;
            if first == 0 {
                first = ptr as PhysicalAddress;
            }
            self.map_existing(access, vaddr, ptr, mmu::page_size());
            vaddr += mmu::page_size() as VirtualAddress;
        }

        first as *mut u8
    }

    pub fn map_existing(&mut self, access: MemoryAccess, vaddr: VirtualAddress, paddr: PhysicalAddress, len: usize) {
        let pages = pages::get_page_area();
        // TODO this readwritexecute is temporary until you get segment data recorded
        self.table.map_addr(access, vaddr, paddr, len, pages).unwrap();
    }

    pub fn unmap_range(&mut self, start: VirtualAddress, len: usize) {
        let pages = pages::get_page_area();
        self.table.unmap_addr(start, len, pages, &|pages, vaddr, paddr| {
            for segment in &self.segments {
                if vaddr >= segment.start && vaddr <= segment.end {
                }
                pages.free_page(paddr as *mut u8);
            }
        }).unwrap();
    }

    pub fn get_ttbr(&self) -> u64 {
        self.table.get_ttbr()
    }
}

