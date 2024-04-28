use super::MBC;

enum BankMode {
    Rom,
    Ram,
}

pub struct MBC1 {
    data: Vec<u8>,
    /// Bank Number
    /// Bank Mode   RAM Bank Bits   ROM Bank Bits
    /// 1 bit       2 bit           5 bit
    bank_number: u8,
    ram_enabled: bool,
    external_ram: Vec<u8>,
}

impl MBC1 {
    #[inline]
    pub fn new(data: Vec<u8>, external_ram: Vec<u8>) -> Self {
        Self {
            data,
            bank_number: 0,
            ram_enabled: false,
            external_ram,
        }
    }

    #[inline]
    fn bank_mode(&self) -> BankMode {
        if self.bank_number & 0b1000_0000 == 0b1000_0000 {
            BankMode::Rom
        } else {
            BankMode::Ram
        }
    }

    #[inline]
    fn rom_bank(&self) -> usize {
        (match self.bank_mode() {
            BankMode::Rom => self.bank_number & 0b0111_1111,
            BankMode::Ram => self.bank_number & 0b0001_1111,
        }) as usize
    }

    #[inline]
    fn ram_bank(&self) -> usize {
        (match self.bank_mode() {
            BankMode::Rom => 0,
            BankMode::Ram => (self.bank_number & 0b0110_0000) >> 5,
        }) as usize
    }
}

impl MBC for MBC1 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // 0000-3FFF - ROM Bank 00 (Read Only)
            // This area always contains the first 16KBytes of the cartridge ROM.
            0x0000..=0x3fff => self.data[addr as usize],

            // 4000-7FFF - ROM Bank 01-7F (Read Only)
            // This area may contain any of the further 16KByte banks of the ROM, allowing to
            // address up to 125 ROM Banks (almost 2MByte). As described below, bank numbers 20h,
            // 40h, and 60h cannot be used, resulting in the odd amount of 125 banks.
            0x4000..=0x5fff => {
                let idx = self.rom_bank() * 0x4000 + (addr as usize - 0x4000);
                self.data[idx]
            }

            // A000-BFFF - RAM Bank 00-03, if any (Read/Write)
            // This area is used to address external RAM in the cartridge (if any). External RAM is
            // often battery buffered, allowing to store game positions or high score tables, even
            // if the gameboy is turned off, or if the cartridge is removed from the gameboy.
            // Available RAM sizes are: 2KByte (at A000-A7FF), 8KByte (at A000-BFFF), and 32KByte
            // (in form of four 8K banks at A000-BFFF).
            0xa000..=0xbfff => {
                if self.ram_enabled {
                    let idx = self.ram_bank() * 0x2000 + (addr as usize - 0xa000);
                    self.external_ram[idx]
                } else {
                    0x00
                }
            }

            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // 0000-1FFF - RAM Enable (Write Only)
            // Before external RAM can be read or written, it must be enabled by writing to this
            // address space. It is recommended to disable external RAM after accessing it, in order
            // to protect its contents from damage during power down of the gameboy. Usually the
            // following values are used:   00h  Disable RAM (default)
            //   0Ah  Enable RAM
            // Practically any value with 0Ah in the lower 4 bits enables RAM, and any other value
            // disables RAM.
            0x0000..=0x1fff => {
                self.ram_enabled = value & 0x0f == 0x0a;
            }

            // 2000-3FFF - ROM Bank Number (Write Only)
            // Writing to this address space selects the lower 5 bits of the ROM Bank Number (in
            // range 01-1Fh). When 00h is written, the MBC translates that to bank 01h also. That
            // doesn't harm so far, because ROM Bank 00h can be always directly accessed by reading
            // from 0000-3FFF. But (when using the register below to specify the upper
            // ROM Bank bits), the same happens for Bank 20h, 40h, and 60h. Any attempt to address
            // these ROM Banks will select Bank 21h, 41h, and 61h instead.
            0x2000..=0x3fff => {
                let lower_bits = match value & 0b0001_1111 {
                    0 => 1,
                    n => n,
                };
                self.bank_number = (self.bank_number & 0b1110_0000) | lower_bits;
            }

            // 4000-5FFF - RAM Bank Number - or - Upper Bits of ROM Bank Number (Write Only) This
            // 2bit register can be used to select a RAM Bank in range from 00-03h, or to specify
            // the upper two bits (Bit 5-6) of the ROM Bank number, depending on the current ROM/RAM
            // Mode. (See below.)
            0x4000..=0x5fff => {
                let n = value & 0x03;
                self.bank_number = self.bank_number & 0b1001_1111 | (n << 5);
            }

            // 6000-7FFF - ROM/RAM Mode Select (Write Only)
            // This 1bit Register selects whether the two bits of the above register should be used
            // as upper two bits of the ROM Bank, or as RAM Bank Number.   00h = ROM
            // Banking Mode (up to 8KByte RAM, 2MByte ROM) (default)   01h = RAM Banking
            // Mode (up to 32KByte RAM, 512KByte ROM) The program may freely switch
            // between both modes, the only limitiation is that only RAM Bank 00h can be used during
            // Mode 0, and only ROM Banks 00-1Fh can be used during Mode 1.
            0x6000..=0x7fff => {
                self.bank_number = self.bank_number & 0b0111_1111 | (value << 7);
            }

            _ => {}
        }
    }
}
