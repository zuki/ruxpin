
use core::mem::MaybeUninit;

use crate::errors::KernelError;
use crate::fs::types::DirEntry;
use crate::misc::copy_struct;
use crate::misc::byteorder::{leu16, leu32};

use super::inodes::Ext2Vnode;

#[repr(C)]
struct Ext2DirEntryHeader {
    inode: leu32,
    entry_len: leu16,
    name_len: u8,
    entry_type: u8,
}

impl Ext2Vnode {
    pub(super) fn read_directory_from_vnode(&mut self, dirent: &mut DirEntry, mut position: usize) -> Result<usize, KernelError> {
        loop {
            // Read the first 8 bytes containing the entry length
            let mut data = [0; 8];
            let nbytes = self.read_from_vnode(&mut data, 8, position)?;
            if nbytes != 8 {
                return Err(KernelError::IOError);
            }

            // Copy the data into a struct to read it
            let entry_on_disk = copy_to_dir_entry_header(&data);
            let entry_len = u16::from(entry_on_disk.entry_len) as usize;

            // If the inode of the not 0, then it's valid, otherwise skip it
            let inode = entry_on_disk.inode.into();
            if inode != 0 {
                dirent.inode = inode;

                // Read the name length bytes into the string buffer
                let nbytes = self.read_from_vnode(dirent.name.as_mut(), entry_on_disk.name_len as usize, position + 8)?;
                if nbytes != entry_on_disk.name_len as usize {
                    return Err(KernelError::IOError);
                }
                unsafe {
                    dirent.name.set_len(entry_on_disk.name_len as usize);
                }

                crate::printkln!("final result {} {}", dirent.inode, dirent.name.as_str());

                return Ok(entry_len);
            }

            position += entry_len;
        }
    }
}

fn copy_to_dir_entry_header(data: &[u8]) -> Ext2DirEntryHeader {
    let mut entry_on_disk: MaybeUninit<Ext2DirEntryHeader> = MaybeUninit::uninit();
    unsafe {
        copy_struct(entry_on_disk.assume_init_mut(), data);
        entry_on_disk.assume_init()
    }
}

