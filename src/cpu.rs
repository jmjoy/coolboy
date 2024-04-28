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

    pub fn alu_add16(&mut self, value1: u16, value2: u16) -> u16 {
        self.f.remove(Flags::Z);
        self.f.remove(Flags::N);
        self.f.set(Flags::H, (value1 & 0xf) + (value2 & 0xf) > 0xf);
        self.f
            .set(Flags::C, (value1 & 0xff) + (value2 & 0xff) > 0xff);
        value1.wrapping_add(value2)
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
}
