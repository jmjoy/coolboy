use std::io::{stdout, Write};

pub struct Serial {
    /// FF01 - SB - Serial transfer data (R/W)
    /// 8 Bits of data to be read/written
    data: u8,

    /// FF02 - SC - Serial Transfer Control (R/W)
    ///   Bit 7 - Transfer Start Flag (0=No Transfer, 1=Start)
    ///   Bit 1 - Clock Speed (0=Normal, 1=Fast) ** CGB Mode Only **
    ///   Bit 0 - Shift Clock (0=External Clock, 1=Internal Clock)
    /// The clock signal specifies the rate at which the eight data bits in SB
    /// (FF01) are transferred. When the gameboy is communicating with another
    /// gameboy (or other computer) then either one must supply internal clock,
    /// and the other one must use external clock.
    control: u8,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            data: 0,
            control: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF01 => self.data,
            0xFF02 => self.control,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF01 => self.data = value,
            0xFF02 => {
                self.control = value;
                if value & 0x81 == 0x81 {
                    // Ignore the internal clock, send all data at once.
                    stdout()
                        .write_all(self.data.to_le_bytes().as_slice())
                        .unwrap();
                    stdout().flush().unwrap();

                    // TODO Inrerrupt
                }
            }
            _ => unreachable!(),
        };
    }
}
