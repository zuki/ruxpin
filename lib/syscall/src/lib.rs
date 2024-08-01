#![no_std]

pub mod arch;
pub use crate::arch::execute_syscall;

use ruxpin_types::{ApiError, FileDesc};

/// システムコール関数
#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SyscallFunction {
    None,

    Exit,
    Fork,
    Exec,
    WaitPid,

    Open,
    Close,
    Read,
    Write,
    ReadDir,
    Dup2,
    Unlink,
    Rename,
    MkDir,
    GetCwd,
    Sync,

    Sbrk,
}

/// システムコール構造体
#[derive(Clone, Debug)]
pub struct SyscallRequest {
    /// システムコール関数
    pub function: SyscallFunction,
    /// 引数（最大6個）
    pub args: [usize; 6],
    /// システムコールの結果
    pub result: usize,
    /// エラーの有無
    pub error: bool,
}

impl Default for SyscallRequest {
    fn default() -> Self {
        Self {
            function: SyscallFunction::None,
            args: [0; 6],
            result: 0,
            error: false,
        }
    }
}


impl SyscallRequest {
    /// システムコールの結果を構造体に設定する
    pub fn store_result(&mut self, result: Result<usize, ApiError>) {
        match result {
            Ok(value) => {
                self.error = false;
                self.result = value;
            },
            Err(value) => {
                self.error = true;
                self.result = ApiError::from(value) as usize;
            },
        }
    }
}

/// システムコールの結果(usize)に変換するトレイト
pub trait IntoSyscallResult {
    fn into_result(self) -> usize;
}

impl IntoSyscallResult for () {
    fn into_result(self) -> usize {
        0
    }
}

impl IntoSyscallResult for bool {
    fn into_result(self) -> usize {
        self as usize
    }
}

impl IntoSyscallResult for i32 {
    fn into_result(self) -> usize {
        self as usize
    }
}

impl IntoSyscallResult for usize {
    fn into_result(self) -> usize {
        self
    }
}

impl IntoSyscallResult for FileDesc {
    fn into_result(self) -> usize {
        self.0 as usize
    }
}

impl<T> IntoSyscallResult for *const T {
    fn into_result(self) -> usize {
        self as usize
    }
}

/// システムコールの結果(usize)をもとの形に変換するトレイト
pub trait FromSyscallResult {
    fn from_result(input: usize) -> Self;
}

impl<T> FromSyscallResult for *const T {
    fn from_result(input: usize) -> Self {
        input as *const T
    }
}

impl FromSyscallResult for () {
    fn from_result(_input: usize) -> Self {
        ()
    }
}

impl FromSyscallResult for bool {
    fn from_result(input: usize) -> Self {
        input != 0
    }
}

impl FromSyscallResult for i32 {
    fn from_result(input: usize) -> Self {
        input as i32
    }
}

impl FromSyscallResult for usize {
    fn from_result(input: usize) -> Self {
        input
    }
}

impl FromSyscallResult for FileDesc {
    fn from_result(input: usize) -> Self {
        FileDesc(input)
    }
}

/// システムコールの引数を設定するするためのマクロ
/// syscall: &mut SystemucallRequest
/// i: 引数の値を示す可変変数
/// name: 引数本体
#[macro_export]
macro_rules! syscall_encode {
    ($syscall:ident, $i:ident, $name:ident: usize) => {
        $i += 1;
        $syscall.args[$i - 1] = $name;
    };

    ($syscall:ident, $i:ident, $name:ident: isize) => {
        $i += 1;
        $syscall.args[$i - 1] = $name as usize;
    };

    ($syscall:ident, $i:ident, $name:ident: Pid) => {
        $i += 1;
        $syscall.args[$i - 1] = $name as usize;
    };

    ($syscall:ident, $i:ident, $name:ident: FileDesc) => {
        $i += 1;
        $syscall.args[$i - 1] = $name.0;
    };

    ($syscall:ident, $i:ident, $name:ident: OpenFlags) => {
        $i += 1;
        $syscall.args[$i - 1] = $name.0 as usize;
    };

    ($syscall:ident, $i:ident, $name:ident: FileAccess) => {
        $i += 1;
        $syscall.args[$i - 1] = $name.0 as usize;
    };

    ($syscall:ident, $i:ident, $name:ident: &[$type:ty]) => {
        $i += 2;
        $syscall.args[$i - 2] = $name.as_ptr() as usize;
        $syscall.args[$i - 1] = $name.len();
    };

    ($syscall:ident, $i:ident, $name:ident: &mut [$type:ty]) => {
        $i += 2;
        $syscall.args[$i - 2] = $name.as_ptr() as usize;
        $syscall.args[$i - 1] = $name.len();
    };

    ($syscall:ident, $i:ident, $name:ident: &str) => {
        $i += 2;
        $syscall.args[$i - 2] = $name.as_bytes().as_ptr() as usize;
        $syscall.args[$i - 1] = $name.as_bytes().len();
    };

    ($syscall:ident, $i:ident, $name:ident: &$type:ty) => {
        $i += 1;
        $syscall.args[$i - 1] = $name as *const $type as *const usize as usize;
    };

    ($syscall:ident, $i:ident, $name:ident: &mut $type:ty) => {
        $i += 1;
        $syscall.args[$i - 1] = $name as *mut $type as *mut usize as usize;
    };
}

/// システムコールの引数から引数の内容を取得するためのマクロ
/// * syscall: &mut SystemucallRequest
/// * i: 引数の値を示す可変変数
/// * name: 引数本体
#[macro_export]
macro_rules! syscall_decode {
    ($syscall:ident, $i:ident, $name:ident: usize) => {
        $i += 1;
        let $name = $syscall.args[$i - 1];
    };

    ($syscall:ident, $i:ident, $name:ident: isize) => {
        $i += 1;
        let $name = $syscall.args[$i - 1] as isize;
    };

    ($syscall:ident, $i:ident, $name:ident: Pid) => {
        $i += 1;
        let $name = $syscall.args[$i - 1] as Pid;
    };

    ($syscall:ident, $i:ident, $name:ident: FileDesc) => {
        $i += 1;
        let $name = FileDesc($syscall.args[$i - 1]);
    };

    ($syscall:ident, $i:ident, $name:ident: OpenFlags) => {
        $i += 1;
        let $name = OpenFlags($syscall.args[$i - 1] as u16);
    };

    ($syscall:ident, $i:ident, $name:ident: FileAccess) => {
        $i += 1;
        let $name = FileAccess($syscall.args[$i - 1] as u16);
    };

    ($syscall:ident, $i:ident, $name:ident: &[$type:ty]) => {
        $i += 2;
        let $name = unsafe {
            core::slice::from_raw_parts($syscall.args[$i - 2] as *const $type, $syscall.args[$i - 1])
        };
    };

    ($syscall:ident, $i:ident, $name:ident: &mut [$type:ty]) => {
        $i += 2;
        let $name = unsafe {
            core::slice::from_raw_parts_mut($syscall.args[$i - 2] as *mut $type, $syscall.args[$i - 1])
        };
    };

    ($syscall:ident, $i:ident, $name:ident: &str) => {
        $i += 2;
        let $name = unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts($syscall.args[$i - 2] as *const u8, $syscall.args[$i - 1]))
        };
    };

    ($syscall:ident, $i:ident, $name:ident: &$type:ty) => {
        $i += 1;
        let $name = unsafe { &*($syscall.args[$i - 1] as *const usize as *const $type) };
    };

    ($syscall:ident, $i:ident, $name:ident: &mut $type:ty) => {
        $i += 1;
        let $name = unsafe { &mut *($syscall.args[$i - 1] as *mut usize as *mut $type) };
    };
}
