use crate::{
    mbc::{new_mbc, MBC},
    serial::Serial,
};
use std::path::Path;

// const MAX_MAP_SIZE: usize = 0x10000;

pub struct MMU {
    mbc: Box<dyn MBC>,
    // map: Box<[u8; MAX_MAP_SIZE]>,
    serial: Serial,
}

impl MMU {
    pub async fn new(mbc: Box<dyn MBC>, serial: Serial) -> anyhow::Result<Self> {
        Ok(Self {
            mbc,
            serial,
            // map: Box::new([0u8; MAX_MAP_SIZE]),
        })
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF01..=0xFF02 => self.serial.read(addr),
            _ => 0,
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read(addr) as u16) | ((self.read(addr + 1) as u16) << 8)
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF01..=0xFF02 => self.serial.write(addr, value),
            _ => {}
        }
    }

    #[inline]
    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write(addr, (value & 0xff) as u8);
        self.write(addr.wrapping_add(1), (value >> 8) as u8);
    }
}
