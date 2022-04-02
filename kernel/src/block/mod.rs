
use alloc::vec::Vec;
use alloc::boxed::Box;
 
use ruxpin_api::types::{OpenFlags, DeviceID, DriverID, SubDeviceID};

use crate::sync::Spinlock;
use crate::errors::KernelError;


pub struct BlockDriver {
    prefix: &'static str,
    devices: Vec<BlockDevice>,
}

pub struct BlockDevice {
    dev: Box<dyn BlockOperations>,
}

pub trait BlockOperations: Sync + Send {
    fn open(&mut self, mode: OpenFlags) -> Result<(), KernelError>;
    fn close(&mut self) -> Result<(), KernelError>;
    fn read(&mut self, buffer: &mut [u8], offset: usize) -> Result<usize, KernelError>;
    fn write(&mut self, buffer: &[u8], offset: usize) -> Result<usize, KernelError>;
    //int (*ioctl)(devminor_t minor, unsigned int request, void *argp, uid_t uid);
    //int (*poll)(devminor_t minor, int events);
    //offset_t (*seek)(devminor_t minor, offset_t position, int whence, offset_t offset);
}


static BLOCK_DRIVERS: Spinlock<Vec<BlockDriver>> = Spinlock::new(Vec::new());


pub fn register_block_driver(prefix: &'static str) -> Result<DriverID, KernelError> {
    let driver_id = BLOCK_DRIVERS.lock().len() as DriverID;
    BLOCK_DRIVERS.lock().push(BlockDriver::new(prefix));
    Ok(driver_id)
}

pub fn read(device: DeviceID, buffer: &mut [u8], offset: usize) -> Result<usize, KernelError> {
    let DeviceID(driver_id, subdevice_id) = device;
    let mut drivers_list = BLOCK_DRIVERS.lock();
    let driver = drivers_list.get_mut(driver_id as usize).ok_or(KernelError::NoSuchDevice)?;
    let device = driver.devices.get_mut(subdevice_id as usize).ok_or(KernelError::NoSuchDevice)?;
    device.dev.read(buffer, offset)
}

pub fn write(device: DeviceID, buffer: &[u8], offset: usize) -> Result<usize, KernelError> {
    let DeviceID(driver_id, subdevice_id) = device;
    let mut drivers_list = BLOCK_DRIVERS.lock();
    let driver = drivers_list.get_mut(driver_id as usize).ok_or(KernelError::NoSuchDevice)?;
    let device = driver.devices.get_mut(subdevice_id as usize).ok_or(KernelError::NoSuchDevice)?;
    device.dev.write(buffer, offset)
}


impl BlockDriver {
    pub const fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, dev: Box<dyn BlockOperations>) -> Result<SubDeviceID, KernelError> {
        let device_id = self.devices.len() as SubDeviceID;
        self.devices.push(BlockDevice::new(dev));
        Ok(device_id)
    }
}

impl BlockDevice {
    pub fn new(dev: Box<dyn BlockOperations>) -> Self {
        Self {
            dev,
        }
    }
}

