
use alloc::vec::Vec;

use crate::mm::pages;
use crate::mm::{MemoryType, MemoryPermissions};
use crate::arch::mmu::{self, TranslationTable};
use crate::arch::types::{VirtualAddress, PhysicalAddress};


const MAX_SEGMENTS: usize = 6;


pub fn init_virtual_memory(start: PhysicalAddress, end: PhysicalAddress) {
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

    pub fn alloc_mapped(&mut self, access: MemoryPermissions, vaddr: VirtualAddress, len: usize) -> *mut u8 {
        let pages = pages::get_page_area();

        self.table.map_addr(MemoryType::Existing, access, vaddr, len, pages, &|pages, _, len| {
            if len == mmu::page_size() {
                Some(pages.alloc_page_zeroed())
            } else {
                None // Don't map granuales larger than a page
            }
        }).unwrap();

        let first = self.table.translate_addr(vaddr).unwrap();
        unsafe {
            first.as_ptr()
        }
    }

    pub fn map_on_demand(&mut self, access: MemoryPermissions, vaddr: VirtualAddress, len: usize) {
        let pages = pages::get_page_area();
        self.table.map_addr(MemoryType::Unallocated, access, vaddr, len, pages, &|_, _, len| {
            if len == mmu::page_size() {
                Some(PhysicalAddress::from(0))
            } else {
                None
            }
        }).unwrap();
    }

    pub fn map_existing(&mut self, access: MemoryPermissions, vaddr: VirtualAddress, paddr: PhysicalAddress, len: usize) {
        let pages = pages::get_page_area();
        self.table.map_addr(MemoryType::Existing, access, vaddr, len, pages, &|_, current_vaddr, _| {
            let voffset = usize::from(current_vaddr) - usize::from(vaddr);
            Some(paddr.add(voffset))
        }).unwrap();
    }

    pub fn unmap_range(&mut self, start: VirtualAddress, len: usize) {
        let pages = pages::get_page_area();
        self.table.unmap_addr(start, len, pages, &|pages, vaddr, paddr| {
            for segment in &self.segments {
                if vaddr >= segment.start && vaddr <= segment.end {
                    // TODO this would normally call the segment operations to determine what to do
                    pages.free_page(paddr);
                }
            }
        }).unwrap();
    }

    pub(crate) fn get_ttbr(&self) -> u64 {
        self.table.get_ttbr()
    }

    pub(crate) fn load_page(&mut self, far: VirtualAddress) {
        //for segment in &self.segments {
        //    if far >= segment.start && far <= segment.end {
                let pages = pages::get_page_area();
                let page = pages.alloc_page_zeroed();
                self.table.update_mapping(far, page, mmu::page_size()).unwrap();
        //        break;
        //    }
        //}
    }
}

