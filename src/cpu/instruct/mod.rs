mod adc;
mod add;
mod and;
mod ccf;
mod cp;
mod cpl;
mod daa;
mod dec;
mod di;
mod ei;
mod halt;
mod inc;
mod jp;
mod jr;
mod ld;
mod nop;
mod or;
mod pop;
mod push;
mod rla;
mod rlca;
mod rra;
mod rrca;
mod sbc;
mod scf;
mod sub;
mod xor;

use self::{
    adc::ADC, add::ADD, and::AND, ccf::CCF, cp::CP, cpl::CPL, daa::DAA, dec::DEC, di::DI, ei::EI,
    halt::HALT, inc::INC, jp::JP, jr::JR, ld::LD, nop::NOP, or::OR, pop::POP, push::PUSH, rla::RLA,
    rlca::RLCA, rra::RRA, rrca::RRCA, sbc::SBC, scf::SCF, sub::SUB, xor::XOR,
};
use crate::cpu::CPU;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
trait InstructionControl {
    fn call(self, cpu: &mut CPU) -> usize;
}

#[enum_dispatch(InstructionControl)]
enum Instruction {
    NOP(NOP),
    LD(LD),
    PUSH(PUSH),
    POP(POP),
    ADD(ADD),
    ADC(ADC),
    SUB(SUB),
    SBC(SBC),
    AND(AND),
    OR(OR),
    XOR(XOR),
    CP(CP),
    INC(INC),
    DEC(DEC),
    DAA(DAA),
    CPL(CPL),
    CCF(CCF),
    SCF(SCF),
    HALT(HALT),
    DI(DI),
    EI(EI),
    RLCA(RLCA),
    RLA(RLA),
    RRCA(RRCA),
    RRA(RRA),
    JP(JP),
    JR(JR),
}

impl Instruction {
    pub fn fetch(cpu: &mut CPU) -> Self {
        let opcode = cpu.fetch();
        match opcode {
            0x00 => Self::NOP(NOP),
            0x06 => Self::LD(LD(Param::Reg(Reg::B), Param::Imm(Imm::U8(cpu.fetch())))),
            0x0e => Self::LD(LD(Param::Reg(Reg::C), Param::Imm(Imm::U8(cpu.fetch())))),
            0x16 => Self::LD(LD(Param::Reg(Reg::D), Param::Imm(Imm::U8(cpu.fetch())))),
            0x1e => Self::LD(LD(Param::Reg(Reg::E), Param::Imm(Imm::U8(cpu.fetch())))),
            0x22 => Self::LD(LD(Param::Addr(Addr::HLInc), Param::Reg(Reg::A))),
            0x26 => Self::LD(LD(Param::Reg(Reg::H), Param::Imm(Imm::U8(cpu.fetch())))),
            0x2e => Self::LD(LD(Param::Reg(Reg::L), Param::Imm(Imm::U8(cpu.fetch())))),
            0x36 => Self::LD(LD(Param::Addr(Addr::HL), Param::Imm(Imm::U8(cpu.fetch())))),
            0x3e => Self::LD(LD(Param::Reg(Reg::A), Param::Imm(Imm::U8(cpu.fetch())))),
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
            0xe2 => Self::LD(LD(Param::Addr(Addr::C), Param::Reg(Reg::A))),
            0xf2 => Self::LD(LD(Param::Reg(Reg::A), Param::Addr(Addr::C))),
            0xc5 => Self::PUSH(PUSH(Reg::BC)),
            0xd5 => Self::PUSH(PUSH(Reg::DE)),
            0xe5 => Self::PUSH(PUSH(Reg::HL)),
            0xf5 => Self::PUSH(PUSH(Reg::AF)),
            0xc1 => Self::POP(POP(Reg::BC)),
            0xd1 => Self::POP(POP(Reg::DE)),
            0xe1 => Self::POP(POP(Reg::HL)),
            0xf1 => Self::POP(POP(Reg::AF)),
            0x80 => Self::ADD(ADD(Param::Reg(Reg::A), Param::Reg(Reg::B))),
            0x81 => Self::ADD(ADD(Param::Reg(Reg::A), Param::Reg(Reg::C))),
            0x82 => Self::ADD(ADD(Param::Reg(Reg::A), Param::Reg(Reg::D))),
            0x83 => Self::ADD(ADD(Param::Reg(Reg::A), Param::Reg(Reg::E))),
            0x84 => Self::ADD(ADD(Param::Reg(Reg::A), Param::Reg(Reg::H))),
            0x85 => Self::ADD(ADD(Param::Reg(Reg::A), Param::Reg(Reg::L))),
            0x86 => Self::ADD(ADD(Param::Reg(Reg::A), Param::Addr(Addr::HL))),
            0x87 => Self::ADD(ADD(Param::Reg(Reg::A), Param::Reg(Reg::A))),
            0xc6 => Self::ADD(ADD(Param::Reg(Reg::A), Param::Imm(Imm::U8(cpu.fetch())))),
            0x88 => Self::ADC(ADC(Param::Reg(Reg::A), Param::Reg(Reg::B))),
            0x89 => Self::ADC(ADC(Param::Reg(Reg::A), Param::Reg(Reg::C))),
            0x8a => Self::ADC(ADC(Param::Reg(Reg::A), Param::Reg(Reg::D))),
            0x8b => Self::ADC(ADC(Param::Reg(Reg::A), Param::Reg(Reg::E))),
            0x8c => Self::ADC(ADC(Param::Reg(Reg::A), Param::Reg(Reg::H))),
            0x8d => Self::ADC(ADC(Param::Reg(Reg::A), Param::Reg(Reg::L))),
            0x8e => Self::ADC(ADC(Param::Reg(Reg::A), Param::Addr(Addr::HL))),
            0x8f => Self::ADC(ADC(Param::Reg(Reg::A), Param::Reg(Reg::A))),
            0xce => Self::ADC(ADC(Param::Reg(Reg::A), Param::Imm(Imm::U8(cpu.fetch())))),
            0x90 => Self::SUB(SUB(Param::Reg(Reg::A), Param::Reg(Reg::B))),
            0x91 => Self::SUB(SUB(Param::Reg(Reg::A), Param::Reg(Reg::C))),
            0x92 => Self::SUB(SUB(Param::Reg(Reg::A), Param::Reg(Reg::D))),
            0x93 => Self::SUB(SUB(Param::Reg(Reg::A), Param::Reg(Reg::E))),
            0x94 => Self::SUB(SUB(Param::Reg(Reg::A), Param::Reg(Reg::H))),
            0x95 => Self::SUB(SUB(Param::Reg(Reg::A), Param::Reg(Reg::L))),
            0x96 => Self::SUB(SUB(Param::Reg(Reg::A), Param::Addr(Addr::HL))),
            0x97 => Self::SUB(SUB(Param::Reg(Reg::A), Param::Reg(Reg::A))),
            0xd6 => Self::SUB(SUB(Param::Reg(Reg::A), Param::Imm(Imm::U8(cpu.fetch())))),
            0x98 => Self::SBC(SBC(Param::Reg(Reg::A), Param::Reg(Reg::B))),
            0x99 => Self::SBC(SBC(Param::Reg(Reg::A), Param::Reg(Reg::C))),
            0x9a => Self::SBC(SBC(Param::Reg(Reg::A), Param::Reg(Reg::D))),
            0x9b => Self::SBC(SBC(Param::Reg(Reg::A), Param::Reg(Reg::E))),
            0x9c => Self::SBC(SBC(Param::Reg(Reg::A), Param::Reg(Reg::H))),
            0x9d => Self::SBC(SBC(Param::Reg(Reg::A), Param::Reg(Reg::L))),
            0x9e => Self::SBC(SBC(Param::Reg(Reg::A), Param::Addr(Addr::HL))),
            0x9f => Self::SBC(SBC(Param::Reg(Reg::A), Param::Reg(Reg::A))),
            0xde => Self::SBC(SBC(Param::Reg(Reg::A), Param::Imm(Imm::U8(cpu.fetch())))),
            0xa0 => Self::AND(AND(Param::Reg(Reg::A), Param::Reg(Reg::B))),
            0xa1 => Self::AND(AND(Param::Reg(Reg::A), Param::Reg(Reg::C))),
            0xa2 => Self::AND(AND(Param::Reg(Reg::A), Param::Reg(Reg::D))),
            0xa3 => Self::AND(AND(Param::Reg(Reg::A), Param::Reg(Reg::E))),
            0xa4 => Self::AND(AND(Param::Reg(Reg::A), Param::Reg(Reg::H))),
            0xa5 => Self::AND(AND(Param::Reg(Reg::A), Param::Reg(Reg::L))),
            0xa6 => Self::AND(AND(Param::Reg(Reg::A), Param::Addr(Addr::HL))),
            0xa7 => Self::AND(AND(Param::Reg(Reg::A), Param::Reg(Reg::A))),
            0xe6 => Self::AND(AND(Param::Reg(Reg::A), Param::Imm(Imm::U8(cpu.fetch())))),
            0xb0 => Self::OR(OR(Param::Reg(Reg::A), Param::Reg(Reg::B))),
            0xb1 => Self::OR(OR(Param::Reg(Reg::A), Param::Reg(Reg::C))),
            0xb2 => Self::OR(OR(Param::Reg(Reg::A), Param::Reg(Reg::D))),
            0xb3 => Self::OR(OR(Param::Reg(Reg::A), Param::Reg(Reg::E))),
            0xb4 => Self::OR(OR(Param::Reg(Reg::A), Param::Reg(Reg::H))),
            0xb5 => Self::OR(OR(Param::Reg(Reg::A), Param::Reg(Reg::L))),
            0xb6 => Self::OR(OR(Param::Reg(Reg::A), Param::Addr(Addr::HL))),
            0xb7 => Self::OR(OR(Param::Reg(Reg::A), Param::Reg(Reg::A))),
            0xf6 => Self::OR(OR(Param::Reg(Reg::A), Param::Imm(Imm::U8(cpu.fetch())))),
            0xa8 => Self::XOR(XOR(Param::Reg(Reg::A), Param::Reg(Reg::B))),
            0xa9 => Self::XOR(XOR(Param::Reg(Reg::A), Param::Reg(Reg::C))),
            0xaa => Self::XOR(XOR(Param::Reg(Reg::A), Param::Reg(Reg::D))),
            0xab => Self::XOR(XOR(Param::Reg(Reg::A), Param::Reg(Reg::E))),
            0xac => Self::XOR(XOR(Param::Reg(Reg::A), Param::Reg(Reg::H))),
            0xad => Self::XOR(XOR(Param::Reg(Reg::A), Param::Reg(Reg::L))),
            0xae => Self::XOR(XOR(Param::Reg(Reg::A), Param::Addr(Addr::HL))),
            0xaf => Self::XOR(XOR(Param::Reg(Reg::A), Param::Reg(Reg::A))),
            0xee => Self::XOR(XOR(Param::Reg(Reg::A), Param::Imm(Imm::U8(cpu.fetch())))),
            0xb8 => Self::CP(CP(Param::Reg(Reg::A), Param::Reg(Reg::B))),
            0xb9 => Self::CP(CP(Param::Reg(Reg::A), Param::Reg(Reg::C))),
            0xba => Self::CP(CP(Param::Reg(Reg::A), Param::Reg(Reg::D))),
            0xbb => Self::CP(CP(Param::Reg(Reg::A), Param::Reg(Reg::E))),
            0xbc => Self::CP(CP(Param::Reg(Reg::A), Param::Reg(Reg::H))),
            0xbd => Self::CP(CP(Param::Reg(Reg::A), Param::Reg(Reg::L))),
            0xbe => Self::CP(CP(Param::Reg(Reg::A), Param::Addr(Addr::HL))),
            0xbf => Self::CP(CP(Param::Reg(Reg::A), Param::Reg(Reg::A))),
            0xfe => Self::CP(CP(Param::Reg(Reg::A), Param::Imm(Imm::U8(cpu.fetch())))),
            0x04 => Self::INC(INC(Param::Reg(Reg::B))),
            0x0c => Self::INC(INC(Param::Reg(Reg::C))),
            0x14 => Self::INC(INC(Param::Reg(Reg::D))),
            0x1c => Self::INC(INC(Param::Reg(Reg::E))),
            0x24 => Self::INC(INC(Param::Reg(Reg::H))),
            0x2c => Self::INC(INC(Param::Reg(Reg::L))),
            0x34 => Self::INC(INC(Param::Addr(Addr::HL))),
            0x3c => Self::INC(INC(Param::Reg(Reg::A))),
            0x05 => Self::DEC(DEC(Param::Reg(Reg::B))),
            0x0d => Self::DEC(DEC(Param::Reg(Reg::C))),
            0x15 => Self::DEC(DEC(Param::Reg(Reg::D))),
            0x1d => Self::DEC(DEC(Param::Reg(Reg::E))),
            0x25 => Self::DEC(DEC(Param::Reg(Reg::H))),
            0x2d => Self::DEC(DEC(Param::Reg(Reg::L))),
            0x35 => Self::DEC(DEC(Param::Addr(Addr::HL))),
            0x3d => Self::DEC(DEC(Param::Reg(Reg::A))),
            0x09 => Self::ADD(ADD(Param::Reg(Reg::HL), Param::Reg(Reg::BC))),
            0x19 => Self::ADD(ADD(Param::Reg(Reg::HL), Param::Reg(Reg::DE))),
            0x29 => Self::ADD(ADD(Param::Reg(Reg::HL), Param::Reg(Reg::HL))),
            0x39 => Self::ADD(ADD(Param::Reg(Reg::HL), Param::Reg(Reg::SP))),
            0xe8 => Self::ADD(ADD(Param::Reg(Reg::SP), Param::Imm(Imm::U8(cpu.fetch())))),
            0x03 => Self::INC(INC(Param::Reg(Reg::BC))),
            0x13 => Self::INC(INC(Param::Reg(Reg::DE))),
            0x23 => Self::INC(INC(Param::Reg(Reg::HL))),
            0x33 => Self::INC(INC(Param::Reg(Reg::SP))),
            0x0b => Self::DEC(DEC(Param::Reg(Reg::BC))),
            0x1b => Self::DEC(DEC(Param::Reg(Reg::DE))),
            0x2b => Self::DEC(DEC(Param::Reg(Reg::HL))),
            0x3b => Self::DEC(DEC(Param::Reg(Reg::SP))),
            0x27 => Self::DAA(DAA),
            0x2f => Self::CPL(CPL),
            0x3f => Self::CCF(CCF),
            0x37 => Self::SCF(SCF),
            0x76 => Self::HALT(HALT),
            0xf3 => Self::DI(DI),
            0xfb => Self::EI(EI),
            0x07 => Self::RLCA(RLCA),
            0x17 => Self::RLA(RLA),
            0x0f => Self::RRCA(RRCA),
            0x1f => Self::RRA(RRA),
            0xc3 => Self::JP(JP(None, cpu.fetch_word())),
            0xc2 => Self::JP(JP(Some(Cond::Z), cpu.fetch_word())),
            0xca => Self::JP(JP(Some(Cond::NZ), cpu.fetch_word())),
            0xd2 => Self::JP(JP(Some(Cond::C), cpu.fetch_word())),
            0xda => Self::JP(JP(Some(Cond::NC), cpu.fetch_word())),
            0x18 => Self::JR(JR(None, cpu.fetch())),
            0x20 => Self::JR(JR(Some(Cond::Z), cpu.fetch())),
            0x28 => Self::JR(JR(Some(Cond::NZ), cpu.fetch())),
            0x30 => Self::JR(JR(Some(Cond::C), cpu.fetch())),
            0x38 => Self::JR(JR(Some(Cond::NC), cpu.fetch())),
            _ => unreachable!("unknown opcode {opcode:x}"),
        }
    }
}

enum Param {
    Reg(Reg),
    Addr(Addr),
    Imm(Imm),
    /// register + (signed 8-bits directly number)
    RegI8(RegI8),
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
    AF,
    SP,
}

enum Addr {
    HL,
    BC,
    DE,
    C,
    HLInc,
    HLDec,
    U8(u8),
    U16(u16),
}

enum Imm {
    U8(u8),
    U16(u16),
}

enum RegI8 {
    SP(u8),
}

enum Cond {
    Z,
    NZ,
    C,
    NC,
}
