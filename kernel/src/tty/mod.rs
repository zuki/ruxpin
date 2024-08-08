
use alloc::vec::Vec;
use alloc::boxed::Box;
 
use ruxpin_syscall::SyscallFunction;
use ruxpin_types::{OpenFlags, DeviceID, DriverID, MinorDeviceID};

use crate::tasklets;
use crate::proc::scheduler;
use crate::sync::Spinlock;
use crate::errors::KernelError;

mod canonical;
use self::canonical::CanonicalReader;

/// キャラクタ型デバイスのOPSトレイト
pub trait CharOperations: Sync + Send {
    fn open(&mut self, mode: OpenFlags) -> Result<(), KernelError>;
    fn close(&mut self) -> Result<(), KernelError>;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, KernelError>;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, KernelError>;
    //int (*ioctl)(devminor_t minor, unsigned int request, void *argp, uid_t uid);
    //int (*poll)(devminor_t minor, int events);
    //offset_t (*seek)(devminor_t minor, offset_t position, int whence, offset_t offset);
}

/// キャラクタ型デバイスドライバ構造体
pub struct CharDriver {
    prefix: &'static str,
    devices: Vec<TtyDevice>,
}

/// TTYデバイス構造体
pub struct TtyDevice {
    dev: Box<dyn CharOperations>,
    reader: Option<CanonicalReader>,
}

/// TTYドライバリスト (Singleton)
static TTY_DRIVERS: Spinlock<Vec<CharDriver>> = Spinlock::new(Vec::new());

/// ttyドライバの登録
pub fn register_tty_driver(prefix: &'static str) -> Result<DriverID, KernelError> {
    let driver_id = TTY_DRIVERS.lock().len() as DriverID;
    TTY_DRIVERS.lock().push(CharDriver::new(prefix));
    Ok(driver_id)
}

/// ttyドライバにデバイス(ops)を登録
pub fn register_tty_device(driver_id: DriverID, dev: Box<dyn CharOperations>) -> Result<MinorDeviceID, KernelError> {
    let mut drivers_list = TTY_DRIVERS.lock();
    let driver = drivers_list.get_mut(driver_id as usize).ok_or(KernelError::NoSuchDevice)?;
    driver.add_device(dev)
}

/// デバイス名のprefixでデバイスを検索
pub fn lookup_device(name: &str) -> Result<DeviceID, KernelError> {
    let drivers_list = TTY_DRIVERS.lock();
    for (driver_id, driver) in drivers_list.iter().enumerate() {
        if driver.prefix == &name[..driver.prefix.len()] {
            let subdevice_id = name[driver.prefix.len()..].parse::<MinorDeviceID>().map_err(|_| KernelError::NoSuchDevice)?;
            if (subdevice_id as usize) < driver.devices.len() {
                return Ok(DeviceID(driver_id as DriverID, subdevice_id));
            }
            break;
        }
    }
    Err(KernelError::NoSuchDevice)
}

/// デバイスをオープン
pub fn open(device_id: DeviceID, mode: OpenFlags) -> Result<(), KernelError> {
    let mut drivers_list = TTY_DRIVERS.lock();
    let device = get_device(&mut *drivers_list, device_id)?;
    device.dev.open(mode)
}

/// デバイスをクローズ
pub fn close(device_id: DeviceID) -> Result<(), KernelError> {
    let mut drivers_list = TTY_DRIVERS.lock();
    let device = get_device(&mut *drivers_list, device_id)?;
    device.dev.close()
}

/// デバイスをRead（デバイスのread関数でread）
#[allow(dead_code)]
pub(crate) fn read_raw(device_id: DeviceID, buffer: &mut [u8]) -> Result<usize, KernelError> {
    let mut drivers_list = TTY_DRIVERS.lock();
    let device = get_device(&mut *drivers_list, device_id)?;
    device.dev.read(buffer)
}

/// デバイスをRead（デバイスのreaderのread関数でread）.
/// 読み込むべきデータが無かった場合はカレントタスクをsuspend
pub fn read(device_id: DeviceID, buffer: &mut [u8]) -> Result<usize, KernelError> {
    let mut drivers_list = TTY_DRIVERS.lock();
    let device = get_device(&mut *drivers_list, device_id)?;

    let nbytes = if let Some(reader) = device.reader.as_mut() {
        reader.read(buffer)?
    } else {
        device.dev.read(buffer)?
    };

    // 読み込むべきデータが無かった場合はカレントタスクをsuspend
    if nbytes == 0 {
        let current = scheduler::get_current();
        tasklets::schedule(Box::new(move || {
            scheduler::suspend(current);
            Ok(())
        }));
    }

    Ok(nbytes)
}

/// デバイスにwrite
pub fn write(device_id: DeviceID, buffer: &[u8]) -> Result<usize, KernelError> {
    let mut drivers_list = TTY_DRIVERS.lock();
    let device = get_device(&mut *drivers_list, device_id)?;
    device.dev.write(buffer)
}

/// デバイスのreaderを再スケジュール
pub fn schedule_update(device_id: DeviceID) {
    tasklets::schedule(Box::new(move || {
        let mut drivers_list = TTY_DRIVERS.lock();
        let device = get_device(&mut *drivers_list, device_id)?;
        if let Some(reader) = device.reader.as_mut() {
            process_input(reader, &mut *device.dev)?;
        }
        Ok(())
    }));
}

/// readerの読み込みを再開
fn process_input(reader: &mut CanonicalReader, dev: &mut dyn CharOperations) -> Result<(), KernelError> {
    let mut ch = [0; 1];
    while dev.read(&mut ch)? > 0 {
        if reader.process_char(dev, ch[0])? {
            scheduler::restart_blocked(SyscallFunction::Read);
            break;
        }
    }
    Ok(())
}

/// device_diのデバイスを取得
fn get_device(drivers_list: &mut Vec<CharDriver>, device_id: DeviceID) -> Result<&mut TtyDevice, KernelError> {
    let DeviceID(driver_id, subdevice_id) = device_id;
    let driver = drivers_list.get_mut(driver_id as usize).ok_or(KernelError::NoSuchDevice)?;
    let device = driver.devices.get_mut(subdevice_id as usize).ok_or(KernelError::NoSuchDevice)?;
    Ok(device)
}

/// キャラクタ型デバイスドライバの実装
impl CharDriver {
    pub const fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, dev: Box<dyn CharOperations>) -> Result<MinorDeviceID, KernelError> {
        let device_id = self.devices.len() as MinorDeviceID;
        self.devices.push(TtyDevice::new(dev));
        Ok(device_id)
    }
}

/// TTYデバイスの実装
impl TtyDevice {
    pub fn new(dev: Box<dyn CharOperations>) -> Self {
        Self {
            dev,
            reader: Some(CanonicalReader::new()),
        }
    }
}

