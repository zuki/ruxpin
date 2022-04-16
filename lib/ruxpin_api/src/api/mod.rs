
use core::fmt;
use core::fmt::Write;

use crate::syscall_encode;
use crate::arch::execute_syscall;
use crate::types::{Pid, FileDesc, ApiError, OpenFlags, FileAccess};
use crate::syscalls::{SyscallRequest, SyscallFunction};



pub fn exit(status: usize) {
    let mut i = 0;
    let mut syscall: SyscallRequest = Default::default();
    syscall_encode!(syscall, i, status: usize);
    syscall.function = SyscallFunction::Exit;
    execute_syscall(&mut syscall);
}

pub fn fork() -> Result<Pid, ApiError> {
    let mut syscall: SyscallRequest = Default::default();
    syscall.function = SyscallFunction::Fork;
    execute_syscall(&mut syscall);
    match syscall.error {
        false => Ok(syscall.result as Pid),
        true => Err(ApiError::SomethingWentWrong),
    }
}

pub fn exec(path: &str) {
    let mut i = 0;
    let mut syscall: SyscallRequest = Default::default();
    syscall_encode!(syscall, i, path: &str);
    syscall.function = SyscallFunction::Exec;
    execute_syscall(&mut syscall);
}


pub fn open(path: &str, flags: OpenFlags, access: FileAccess) -> Result<FileDesc, ApiError> {
    let mut i = 0;
    let mut syscall: SyscallRequest = Default::default();
    syscall_encode!(syscall, i, path: &str);
    syscall_encode!(syscall, i, flags: OpenFlags);
    syscall_encode!(syscall, i, access: FileAccess);
    syscall.function = SyscallFunction::Open;
    execute_syscall(&mut syscall);
    match syscall.error {
        false => Ok(FileDesc(syscall.result)),
        true => Err(ApiError::SomethingWentWrong),
    }
}

pub fn close(file: FileDesc) -> Result<(), ApiError> {
    let mut i = 0;
    let mut syscall: SyscallRequest = Default::default();
    syscall_encode!(syscall, i, file: FileDesc);
    syscall.function = SyscallFunction::Close;
    execute_syscall(&mut syscall);
    match syscall.error {
        false => Ok(()),
        true => Err(ApiError::SomethingWentWrong),
    }
}

pub fn read(file: FileDesc, buffer: &mut [u8]) -> Result<usize, ApiError> {
    let mut i = 0;
    let mut syscall: SyscallRequest = Default::default();
    syscall_encode!(syscall, i, file: FileDesc);
    syscall_encode!(syscall, i, buffer: &mut [u8]);
    syscall.function = SyscallFunction::Read;
    execute_syscall(&mut syscall);
    match syscall.error {
        false => Ok(syscall.result),
        true => Err(ApiError::SomethingWentWrong),
    }
}

pub fn write(file: FileDesc, buffer: &[u8]) -> Result<usize, ApiError> {
    let mut i = 0;
    let mut syscall: SyscallRequest = Default::default();
    syscall_encode!(syscall, i, file: FileDesc);
    syscall_encode!(syscall, i, buffer: &[u8]);
    syscall.function = SyscallFunction::Write;
    execute_syscall(&mut syscall);
    match syscall.error {
        false => Ok(syscall.result),
        true => Err(ApiError::SomethingWentWrong),
    }
}


impl Write for FileDesc {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(*self, s.as_bytes()).unwrap();
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => ({
        use core::fmt::Write;
        FileDesc(0).write_fmt(format_args!($($args)*)).unwrap();
    })
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => ({
        use core::fmt::Write;
        FileDesc(0).write_fmt(format_args!($($args)*)).unwrap();
        FileDesc(0).write_str("\n").unwrap();
    })
}

