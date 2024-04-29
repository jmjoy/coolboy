use crate::{
    clock::ClockSyncer,
    instruct::{self, Instruction, InstructionControl},
    mmu::MMU,
};
use bitflags::bitflags;
use std::num::Wrapping;

pub struct Register {
    pub a: Wrapping<u8>,
    pub b: Wrapping<u8>,
    pub c: Wrapping<u8>,
    pub d: Wrapping<u8>,
    pub e: Wrapping<u8>,
    /// flag register
    pub f: Flags,
    pub h: Wrapping<u8>,
    pub l: Wrapping<u8>,
    /// program counter
    pub pc: Wrapping<u16>,
    /// stack pointer
    pub sp: Wrapping<u16>,
}

impl Register {
    #[inline]
    pub fn new() -> Self {
        Self {
            a: Wrapping(0),
            b: Wrapping(0),
            c: Wrapping(0),
            d: Wrapping(0),
            e: Wrapping(0),
            f: Flags::empty(),
            h: Wrapping(0),
            l: Wrapping(0),
            pc: Wrapping(0x0100),
            sp: Wrapping(0xFFFE),
        }
    }

    #[inline]
    pub fn get_hl(&self) -> u16 {
        (self.h.0 as u16) << 8 | (self.l.0 as u16)
    }

    #[inline]
    pub fn set_hl(&mut self, value: u16) {
        self.h.0 = (value >> 8) as u8;
        self.l.0 = value as u8;
    }

    #[inline]
    pub fn get_bc(&self) -> u16 {
        (self.b.0 as u16) << 8 | (self.c.0 as u16)
    }

    #[inline]
    pub fn set_bc(&mut self, value: u16) {
        self.b.0 = (value >> 8) as u8;
        self.c.0 = value as u8;
    }

    #[inline]
    pub fn get_de(&self) -> u16 {
        (self.d.0 as u16) << 8 | (self.e.0 as u16)
    }

    #[inline]
    pub fn set_de(&mut self, value: u16) {
        self.d.0 = (value >> 8) as u8;
        self.e.0 = value as u8;
    }

    #[inline]
    pub fn get_af(&self) -> u16 {
        (self.a.0 as u16) << 8 | (self.f.bits() as u16)
    }

    #[inline]
    pub fn set_af(&mut self, value: u16) {
        self.a.0 = (value >> 8) as u8;
        self.f = Flags::from_bits_retain(value as u8);
    }

    pub fn alu_add(&mut self, value1: u8, value2: u8) -> u8 {
        let (value, overflow) = value1.overflowing_add(value2);
        self.f.set(Flags::Z, value == 0);
        self.f.remove(Flags::N);
        self.f.set(Flags::H, (value1 & 0xf) + (value2 & 0xf) > 0xf);
        self.f.set(Flags::C, overflow);
        value
    }

    pub fn alu_adc(&mut self, value1: u8, value2: u8) -> u8 {
        let c = self.f.contains(Flags::C) as u8;
        let (value, overflow) = value1.overflowing_add(value2);
        let (value, overflow2) = value.overflowing_add(c);
        self.f.set(Flags::Z, value == 0);
        self.f.remove(Flags::N);
        self.f
            .set(Flags::H, (value1 & 0xf) + (value2 & 0xf) + (c & 0xf) > 0xf);
        self.f.set(Flags::C, overflow || overflow2);
        value
    }

    pub fn alu_sub(&mut self, value1: u8, value2: u8) -> u8 {
        let (value, overflow) = value1.overflowing_sub(value2);
        self.f.set(Flags::Z, value == 0);
        self.f.insert(Flags::N);
        self.f.set(Flags::H, (value1 & 0xf) < (value2 & 0xf));
        self.f.set(Flags::C, overflow);
        value
    }

    pub fn alu_sbc(&mut self, value1: u8, value2: u8) -> u8 {
        let c = self.f.contains(Flags::C) as u8;
        let (value, overflow) = value1.overflowing_sub(value2);
        let (value, overflow2) = value.overflowing_sub(c);
        self.f.set(Flags::Z, value == 0);
        self.f.insert(Flags::N);
        self.f
            .set(Flags::H, (value1 & 0xf) < (value2 & 0xf) + (c & 0xf));
        self.f.set(Flags::C, overflow || overflow2);
        value
    }

    pub fn alu_and(&mut self, value1: u8, value2: u8) -> u8 {
        let value = value1 & value2;
        self.f.set(Flags::Z, value == 0);
        self.f.remove(Flags::N);
        self.f.insert(Flags::H);
        self.f.remove(Flags::C);
        value
    }

    pub fn alu_or(&mut self, value1: u8, value2: u8) -> u8 {
        let value = value1 | value2;
        self.f.set(Flags::Z, value == 0);
        self.f.remove(Flags::N);
        self.f.remove(Flags::H);
        self.f.remove(Flags::C);
        value
    }

    pub fn alu_xor(&mut self, value1: u8, value2: u8) -> u8 {
        let value = value1 ^ value2;
        self.f.set(Flags::Z, value == 0);
        self.f.remove(Flags::N);
        self.f.remove(Flags::H);
        self.f.remove(Flags::C);
        value
    }

    pub fn alu_inc(&mut self, value1: u8) -> u8 {
        let value = value1.wrapping_add(1);
        self.f.set(Flags::Z, value == 0);
        self.f.remove(Flags::N);
        self.f.set(Flags::H, (value1 & 0xf) + 1 > 0xf);
        value
    }

    pub fn alu_dec(&mut self, value1: u8) -> u8 {
        let value = value1.wrapping_sub(1);
        self.f.set(Flags::Z, value == 0);
        self.f.remove(Flags::N);
        self.f.set(Flags::H, (value1 & 0xf) == 0);
        value
    }

    pub fn alu_add_sp_i8(&mut self, i: u8) -> u16 {
        let sp = self.sp.0;
        let i = i as i8 as i16 as u16;

        self.f.remove(Flags::Z);
        self.f.remove(Flags::N);
        self.f.set(Flags::H, (i & 0xf) + (sp & 0xf) > 0xf);
        self.f.set(Flags::C, (i & 0xff) + (sp & 0xff) > 0xff);

        sp.wrapping_add(i)
    }

    pub fn alu_add_u16(&mut self, value1: u16, value2: u16) -> u16 {
        let (value, overflow) = value1.overflowing_add(value2);
        self.f.remove(Flags::N);
        self.f
            .set(Flags::H, (value1 & 0xff) + (value2 & 0xff) > 0xff);
        self.f.set(Flags::C, overflow);
        value
    }

    pub fn alu_daa(&mut self, a: u8) -> u8 {
        let mut a = Wrapping(a);

        let mut adjust = if self.f.contains(Flags::C) {
            0x60
        } else {
            0x00
        };
        if self.f.contains(Flags::H) {
            adjust |= 0x06;
        };
        if !self.f.contains(Flags::N) {
            if a.0 & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if a.0 > 0x99 {
                adjust |= 0x60;
            };
            a += adjust;
        } else {
            a -= adjust;
        }

        self.f.set(Flags::Z, a.0 == 0);
        self.f.set(Flags::C, adjust >= 0x60);
        self.f.remove(Flags::H);

        a.0
    }
}

bitflags! {
    /// Represents a set of flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Flags: u8 {
        /// zero flag
        const Z = 0b10000000;
        /// negative flag
        const N = 0b01000000;
        /// half-carry flag
        const H = 0b00100000;
        /// carry flag
        const C = 0b00010000;
    }
}

pub struct CPU {
    pub reg: Register,
    syncer: ClockSyncer,
    instruction: Option<Instruction>,
    remain_ticks: usize,

    pub mmu: MMU,
}

impl CPU {
    pub fn new(syncer: ClockSyncer, mmu: MMU) -> Self {
        Self {
            reg: Register::new(),
            syncer,
            instruction: None,
            remain_ticks: 0,
            mmu,
        }
    }

    pub async fn run(mut self) {
        log::debug!(flags:? = self.reg.f; "cpu register");
        loop {
            self.syncer.receive_tick().await;
            self.tick();
            self.syncer.finish_tick().await;
        }
    }

    fn tick(&mut self) {
        if self.remain_ticks > 0 {
            self.remain_ticks -= 1;
            return;
        }

        self.remain_ticks = Instruction::fetch(self).call(self) * 4;

        // let instruction = self.instruction.take();
        // match instruction {
        //     Some(instruction) => {
        //         self.remain_ticks = instruction.call(self) * 4;
        //     }
        //     None => {
        //         // TODO load instruction
        //         let opcode = self.mmu.read(self.reg.pc.0);
        //         self.instruction = Some(Instruction::new(opcode));
        //         self.reg.pc += 1;

        //         self.remain_ticks += 4;
        //     }
        // }
    }

    pub fn fetch(&mut self) -> u8 {
        let value = self.mmu.read(self.reg.pc.0);
        self.reg.pc += 1;
        value
    }

    pub fn fetch_word(&mut self) -> u16 {
        let value = self.mmu.read_word(self.reg.pc.0);
        self.reg.pc += 2;
        value
    }

    pub fn push_stack(&mut self, value: u16) {
        self.reg.sp -= 2;
        self.mmu.write_word(self.reg.sp.0, value);
    }

    pub fn pop_stack(&mut self) -> u16 {
        let value = self.mmu.read_word(self.reg.sp.0);
        self.reg.sp += 2;
        value
    }
}
