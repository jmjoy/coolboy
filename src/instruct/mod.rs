mod ld;
mod nop;

use self::{ld::LD, nop::NOP};
use crate::cpu::CPU;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait InstructionControl {
    fn call(self, cpu: &mut CPU) -> usize;
}

#[enum_dispatch(InstructionControl)]
pub enum Instruction {
    NOP(NOP),
    LD(LD),
}

impl Instruction {
    pub fn fetch(cpu: &mut CPU) -> Self {
        let opcode = cpu.fetch();
        match opcode {
            0x00 => Self::NOP(NOP),
            0x06 => Self::LD(LD(Param::Reg(Reg::B), Param::N8(cpu.fetch()))),
            0x0e => Self::LD(LD(Param::Reg(Reg::C), Param::N8(cpu.fetch()))),
            0x16 => Self::LD(LD(Param::Reg(Reg::D), Param::N8(cpu.fetch()))),
            0x1e => Self::LD(LD(Param::Reg(Reg::E), Param::N8(cpu.fetch()))),
            0x26 => Self::LD(LD(Param::Reg(Reg::H), Param::N8(cpu.fetch()))),
            0x2e => Self::LD(LD(Param::Reg(Reg::L), Param::N8(cpu.fetch()))),
            0x36 => Self::LD(LD(Param::Addr(Addr::HL), Param::N8(cpu.fetch()))),
            0x3e => Self::LD(LD(Param::Reg(Reg::A), Param::N8(cpu.fetch()))),
            0x40 => Self::LD(LD(Param::Reg(Reg::B), Param::Reg(Reg::B))),
            0x41 => Self::LD(LD(Param::Reg(Reg::B), Param::Reg(Reg::C))),
            0x42 => Self::LD(LD(Param::Reg(Reg::B), Param::Reg(Reg::D))),
            0x43 => Self::LD(LD(Param::Reg(Reg::B), Param::Reg(Reg::E))),
            0x44 => Self::LD(LD(Param::Reg(Reg::B), Param::Reg(Reg::H))),
            0x45 => Self::LD(LD(Param::Reg(Reg::B), Param::Reg(Reg::L))),
            0x46 => Self::LD(LD(Param::Reg(Reg::B), Param::Addr(Addr::HL))),
            0x47 => Self::LD(LD(Param::Reg(Reg::B), Param::Reg(Reg::A))),
            0x48 => Self::LD(LD(Param::Reg(Reg::C), Param::Reg(Reg::B))),
            0x49 => Self::LD(LD(Param::Reg(Reg::C), Param::Reg(Reg::C))),
            0x4a => Self::LD(LD(Param::Reg(Reg::C), Param::Reg(Reg::D))),
            0x4b => Self::LD(LD(Param::Reg(Reg::C), Param::Reg(Reg::E))),
            0x4c => Self::LD(LD(Param::Reg(Reg::C), Param::Reg(Reg::H))),
            0x4d => Self::LD(LD(Param::Reg(Reg::C), Param::Reg(Reg::L))),
            0x4e => Self::LD(LD(Param::Reg(Reg::C), Param::Addr(Addr::HL))),
            0x4f => Self::LD(LD(Param::Reg(Reg::C), Param::Reg(Reg::A))),
            0x50 => Self::LD(LD(Param::Reg(Reg::D), Param::Reg(Reg::B))),
            0x51 => Self::LD(LD(Param::Reg(Reg::D), Param::Reg(Reg::C))),
            0x52 => Self::LD(LD(Param::Reg(Reg::D), Param::Reg(Reg::D))),
            0x53 => Self::LD(LD(Param::Reg(Reg::D), Param::Reg(Reg::E))),
            0x54 => Self::LD(LD(Param::Reg(Reg::D), Param::Reg(Reg::H))),
            0x55 => Self::LD(LD(Param::Reg(Reg::D), Param::Reg(Reg::L))),
            0x56 => Self::LD(LD(Param::Reg(Reg::D), Param::Addr(Addr::HL))),
            0x57 => Self::LD(LD(Param::Reg(Reg::D), Param::Reg(Reg::A))),
            0x58 => Self::LD(LD(Param::Reg(Reg::E), Param::Reg(Reg::B))),
            0x59 => Self::LD(LD(Param::Reg(Reg::E), Param::Reg(Reg::C))),
            0x5a => Self::LD(LD(Param::Reg(Reg::E), Param::Reg(Reg::D))),
            0x5b => Self::LD(LD(Param::Reg(Reg::E), Param::Reg(Reg::E))),
            0x5c => Self::LD(LD(Param::Reg(Reg::E), Param::Reg(Reg::H))),
            0x5d => Self::LD(LD(Param::Reg(Reg::E), Param::Reg(Reg::L))),
            0x5e => Self::LD(LD(Param::Reg(Reg::E), Param::Addr(Addr::HL))),
            0x5f => Self::LD(LD(Param::Reg(Reg::E), Param::Reg(Reg::A))),
            0x60 => Self::LD(LD(Param::Reg(Reg::H), Param::Reg(Reg::B))),
            0x61 => Self::LD(LD(Param::Reg(Reg::H), Param::Reg(Reg::C))),
            0x62 => Self::LD(LD(Param::Reg(Reg::H), Param::Reg(Reg::D))),
            0x63 => Self::LD(LD(Param::Reg(Reg::H), Param::Reg(Reg::E))),
            0x64 => Self::LD(LD(Param::Reg(Reg::H), Param::Reg(Reg::H))),
            0x65 => Self::LD(LD(Param::Reg(Reg::H), Param::Reg(Reg::L))),
            0x66 => Self::LD(LD(Param::Reg(Reg::H), Param::Addr(Addr::HL))),
            0x67 => Self::LD(LD(Param::Reg(Reg::H), Param::Reg(Reg::A))),
            0x68 => Self::LD(LD(Param::Reg(Reg::L), Param::Reg(Reg::B))),
            0x69 => Self::LD(LD(Param::Reg(Reg::L), Param::Reg(Reg::C))),
            0x6a => Self::LD(LD(Param::Reg(Reg::L), Param::Reg(Reg::D))),
            0x6b => Self::LD(LD(Param::Reg(Reg::L), Param::Reg(Reg::E))),
            0x6c => Self::LD(LD(Param::Reg(Reg::L), Param::Reg(Reg::H))),
            0x6d => Self::LD(LD(Param::Reg(Reg::L), Param::Reg(Reg::L))),
            0x6e => Self::LD(LD(Param::Reg(Reg::L), Param::Addr(Addr::HL))),
            0x6f => Self::LD(LD(Param::Reg(Reg::L), Param::Reg(Reg::A))),
            0x70 => Self::LD(LD(Param::Addr(Addr::HL), Param::Reg(Reg::B))),
            0x71 => Self::LD(LD(Param::Addr(Addr::HL), Param::Reg(Reg::C))),
            0x72 => Self::LD(LD(Param::Addr(Addr::HL), Param::Reg(Reg::D))),
            0x73 => Self::LD(LD(Param::Addr(Addr::HL), Param::Reg(Reg::E))),
            0x74 => Self::LD(LD(Param::Addr(Addr::HL), Param::Reg(Reg::H))),
            0x75 => Self::LD(LD(Param::Addr(Addr::HL), Param::Reg(Reg::L))),
            0x77 => Self::LD(LD(Param::Addr(Addr::HL), Param::Reg(Reg::A))),
            0x78 => Self::LD(LD(Param::Reg(Reg::A), Param::Reg(Reg::B))),
            0x79 => Self::LD(LD(Param::Reg(Reg::A), Param::Reg(Reg::C))),
            0x7a => Self::LD(LD(Param::Reg(Reg::A), Param::Reg(Reg::D))),
            0x7b => Self::LD(LD(Param::Reg(Reg::A), Param::Reg(Reg::E))),
            0x7c => Self::LD(LD(Param::Reg(Reg::A), Param::Reg(Reg::H))),
            0x7d => Self::LD(LD(Param::Reg(Reg::A), Param::Reg(Reg::L))),
            0x7e => Self::LD(LD(Param::Reg(Reg::A), Param::Addr(Addr::HL))),
            0x7f => Self::LD(LD(Param::Reg(Reg::A), Param::Reg(Reg::A))),
            0x02 => Self::LD(LD(Param::Addr(Addr::BC), Param::Reg(Reg::A))),
            0x12 => Self::LD(LD(Param::Addr(Addr::DE), Param::Reg(Reg::A))),
            0x0a => Self::LD(LD(Param::Reg(Reg::A), Param::Addr(Addr::BC))),
            0x1a => Self::LD(LD(Param::Reg(Reg::A), Param::Addr(Addr::DE))),

            _ => unreachable!(),
        }
    }
}

enum Param {
    Reg(Reg),
    Addr(Addr),
    N8(u8),
    N16(u16),
    /// register + (signed 8-bits directly number)
    RegS8(RegS8),
}

enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    SP,
}

enum Addr {
    HL,
    BC,
    DE,
    C,
    HL_INC,
    HL_DEC,
    N8(u8),
    N16(u16),
}

enum RegS8 {
    SP(u8),
}
