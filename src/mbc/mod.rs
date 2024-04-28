mod mbc0;
mod mbc1;

use self::{mbc0::MBC0, mbc1::MBC1};
use anyhow::bail;
use std::{num::Wrapping, path::Path};
use tokio::fs;

pub trait MBC: Send {
    fn read(&self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, value: u8);
}

pub async fn new_mbc(path: impl AsRef<Path>) -> anyhow::Result<Box<dyn MBC>> {
    let data = fs::read(path.as_ref()).await?;

    if data.len() < 0x150 {
        bail!("Rom size to small");
    }

    check_checksum(&data)?;

    let mbc: Box<dyn MBC> = match data[0x147] {
        0x00 => Box::new(MBC0::new(data)),
        0x01 => Box::new(MBC1::new(data, vec![])),
        0x02 => Box::new(MBC1::new(data, vec![0u8; 0x2000])),
        0x03 => Box::new(MBC1::new(data, vec![0u8; 0x2000])),
        0x05..=0x06 => todo!("mbc2::MBC2::new(data).map(|v| Box::new(v) as Box<dyn MBC>)"),
        0x0F..=0x13 => todo!("mbc3::MBC3::new(data).map(|v| Box::new(v) as Box<dyn MBC>)"),
        0x19..=0x1E => todo!("mbc5::MBC5::new(data).map(|v| Box::new(v) as Box<dyn MBC>)"),
        _ => bail!("Unsupported MBC type"),
    };

    Ok(mbc)
}

fn check_checksum(data: &[u8]) -> anyhow::Result<()> {
    let mut value = Wrapping(0u8);
    for i in 0x134..0x14D {
        value -= data[i];
        value -= 1;
    }
    if value != Wrapping(data[0x14D]) {
        bail!("Cartridge checksum is invalid");
    }
    Ok(())
}
