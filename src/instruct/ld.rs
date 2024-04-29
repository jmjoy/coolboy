use super::{Addr, Imm, InstructionControl, Param, Reg, RegI8};
use crate::cpu::CPU;
use std::{num::Wrapping, ops::Add};

pub struct LD(pub Param, pub Param);

impl InstructionControl for LD {
    fn call(self, cpu: &mut CPU) -> usize {
        match (self.0, self.1) {
            (Param::Reg(Reg::A), Param::Imm(Imm::U8(n))) => {
                cpu.reg.a = Wrapping(n);
                8
            }
            (Param::Reg(Reg::B), Param::Imm(Imm::U8(n))) => {
                cpu.reg.b = Wrapping(n);
                8
            }
            (Param::Reg(Reg::C), Param::Imm(Imm::U8(n))) => {
                cpu.reg.c = Wrapping(n);
                8
            }
            (Param::Reg(Reg::D), Param::Imm(Imm::U8(n))) => {
                cpu.reg.d = Wrapping(n);
                8
            }
            (Param::Reg(Reg::E), Param::Imm(Imm::U8(n))) => {
                cpu.reg.e = Wrapping(n);
                8
            }
            (Param::Reg(Reg::H), Param::Imm(Imm::U8(n))) => {
                cpu.reg.h = Wrapping(n);
                8
            }
            (Param::Reg(Reg::L), Param::Imm(Imm::U8(n))) => {
                cpu.reg.l = Wrapping(n);
                8
            }
            (Param::Addr(Addr::HL), Param::Imm(Imm::U8(n))) => {
                cpu.mmu.write(cpu.reg.get_hl(), n);
                8
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::A)) => {
                cpu.reg.a = cpu.reg.a;
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::B)) => {
                cpu.reg.a = cpu.reg.b;
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::C)) => {
                cpu.reg.a = cpu.reg.c;
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::D)) => {
                cpu.reg.a = cpu.reg.d;
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::E)) => {
                cpu.reg.a = cpu.reg.e;
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::H)) => {
                cpu.reg.a = cpu.reg.h;
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::L)) => {
                cpu.reg.a = cpu.reg.l;
                4
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::HL)) => {
                cpu.reg.a = Wrapping(cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::BC)) => {
                cpu.reg.a = Wrapping(cpu.mmu.read(cpu.reg.get_bc()));
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::DE)) => {
                cpu.reg.a = Wrapping(cpu.mmu.read(cpu.reg.get_de()));
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::C)) => {
                cpu.reg.a = Wrapping(cpu.mmu.read(cpu.reg.c.0 as u16 | 0xff00));
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::HLInc)) => {
                let hl = cpu.reg.get_hl();
                cpu.reg.a = Wrapping(cpu.mmu.read(hl));
                cpu.reg.set_hl(hl.wrapping_add(1));
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::HLDec)) => {
                let hl = cpu.reg.get_hl();
                cpu.reg.a = Wrapping(cpu.mmu.read(hl));
                cpu.reg.set_hl(hl.wrapping_sub(1));
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::U8(n))) => {
                cpu.reg.a = Wrapping(cpu.mmu.read(0xff00 | n as u16));
                12
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::U16(n))) => {
                cpu.reg.a = Wrapping(cpu.mmu.read(n));
                16
            }
            (Param::Reg(Reg::B), Param::Reg(Reg::A)) => {
                cpu.reg.b = cpu.reg.a;
                4
            }
            (Param::Reg(Reg::B), Param::Reg(Reg::B)) => {
                cpu.reg.b = cpu.reg.b;
                4
            }
            (Param::Reg(Reg::B), Param::Reg(Reg::C)) => {
                cpu.reg.b = cpu.reg.c;
                4
            }
            (Param::Reg(Reg::B), Param::Reg(Reg::D)) => {
                cpu.reg.b = cpu.reg.d;
                4
            }
            (Param::Reg(Reg::B), Param::Reg(Reg::E)) => {
                cpu.reg.b = cpu.reg.e;
                4
            }
            (Param::Reg(Reg::B), Param::Reg(Reg::H)) => {
                cpu.reg.b = cpu.reg.h;
                4
            }
            (Param::Reg(Reg::B), Param::Reg(Reg::L)) => {
                cpu.reg.b = cpu.reg.l;
                4
            }
            (Param::Reg(Reg::B), Param::Addr(Addr::HL)) => {
                cpu.reg.b = Wrapping(cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            (Param::Reg(Reg::C), Param::Reg(Reg::A)) => {
                cpu.reg.c = cpu.reg.a;
                4
            }
            (Param::Reg(Reg::C), Param::Reg(Reg::B)) => {
                cpu.reg.c = cpu.reg.b;
                4
            }
            (Param::Reg(Reg::C), Param::Reg(Reg::C)) => {
                cpu.reg.c = cpu.reg.c;
                4
            }
            (Param::Reg(Reg::C), Param::Reg(Reg::D)) => {
                cpu.reg.c = cpu.reg.d;
                4
            }
            (Param::Reg(Reg::C), Param::Reg(Reg::E)) => {
                cpu.reg.c = cpu.reg.e;
                4
            }
            (Param::Reg(Reg::C), Param::Reg(Reg::H)) => {
                cpu.reg.c = cpu.reg.h;
                4
            }
            (Param::Reg(Reg::C), Param::Reg(Reg::L)) => {
                cpu.reg.c = cpu.reg.l;
                4
            }
            (Param::Reg(Reg::C), Param::Addr(Addr::HL)) => {
                cpu.reg.c = Wrapping(cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            (Param::Reg(Reg::D), Param::Reg(Reg::A)) => {
                cpu.reg.d = cpu.reg.a;
                4
            }
            (Param::Reg(Reg::D), Param::Reg(Reg::B)) => {
                cpu.reg.d = cpu.reg.b;
                4
            }
            (Param::Reg(Reg::D), Param::Reg(Reg::C)) => {
                cpu.reg.d = cpu.reg.c;
                4
            }
            (Param::Reg(Reg::D), Param::Reg(Reg::D)) => {
                cpu.reg.d = cpu.reg.d;
                4
            }
            (Param::Reg(Reg::D), Param::Reg(Reg::E)) => {
                cpu.reg.d = cpu.reg.e;
                4
            }
            (Param::Reg(Reg::D), Param::Reg(Reg::H)) => {
                cpu.reg.d = cpu.reg.h;
                4
            }
            (Param::Reg(Reg::D), Param::Reg(Reg::L)) => {
                cpu.reg.d = cpu.reg.l;
                4
            }
            (Param::Reg(Reg::D), Param::Addr(Addr::HL)) => {
                cpu.reg.d = Wrapping(cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            (Param::Reg(Reg::E), Param::Reg(Reg::A)) => {
                cpu.reg.e = cpu.reg.a;
                4
            }
            (Param::Reg(Reg::E), Param::Reg(Reg::B)) => {
                cpu.reg.e = cpu.reg.b;
                4
            }
            (Param::Reg(Reg::E), Param::Reg(Reg::C)) => {
                cpu.reg.e = cpu.reg.c;
                4
            }
            (Param::Reg(Reg::E), Param::Reg(Reg::D)) => {
                cpu.reg.e = cpu.reg.d;
                4
            }
            (Param::Reg(Reg::E), Param::Reg(Reg::E)) => {
                cpu.reg.e = cpu.reg.e;
                4
            }
            (Param::Reg(Reg::E), Param::Reg(Reg::H)) => {
                cpu.reg.e = cpu.reg.h;
                4
            }
            (Param::Reg(Reg::E), Param::Reg(Reg::L)) => {
                cpu.reg.e = cpu.reg.l;
                4
            }
            (Param::Reg(Reg::E), Param::Addr(Addr::HL)) => {
                cpu.reg.e = Wrapping(cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            (Param::Reg(Reg::H), Param::Reg(Reg::A)) => {
                cpu.reg.h = cpu.reg.a;
                4
            }
            (Param::Reg(Reg::H), Param::Reg(Reg::B)) => {
                cpu.reg.h = cpu.reg.b;
                4
            }
            (Param::Reg(Reg::H), Param::Reg(Reg::C)) => {
                cpu.reg.h = cpu.reg.c;
                4
            }
            (Param::Reg(Reg::H), Param::Reg(Reg::D)) => {
                cpu.reg.h = cpu.reg.d;
                4
            }
            (Param::Reg(Reg::H), Param::Reg(Reg::E)) => {
                cpu.reg.h = cpu.reg.e;
                4
            }
            (Param::Reg(Reg::H), Param::Reg(Reg::H)) => {
                cpu.reg.h = cpu.reg.h;
                4
            }
            (Param::Reg(Reg::H), Param::Reg(Reg::L)) => {
                cpu.reg.h = cpu.reg.l;
                4
            }
            (Param::Reg(Reg::H), Param::Addr(Addr::HL)) => {
                cpu.reg.h = Wrapping(cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            (Param::Reg(Reg::L), Param::Reg(Reg::A)) => {
                cpu.reg.l = cpu.reg.a;
                4
            }
            (Param::Reg(Reg::L), Param::Reg(Reg::B)) => {
                cpu.reg.l = cpu.reg.b;
                4
            }
            (Param::Reg(Reg::L), Param::Reg(Reg::C)) => {
                cpu.reg.l = cpu.reg.c;
                4
            }
            (Param::Reg(Reg::L), Param::Reg(Reg::D)) => {
                cpu.reg.l = cpu.reg.d;
                4
            }
            (Param::Reg(Reg::L), Param::Reg(Reg::E)) => {
                cpu.reg.l = cpu.reg.e;
                4
            }
            (Param::Reg(Reg::L), Param::Reg(Reg::H)) => {
                cpu.reg.l = cpu.reg.h;
                4
            }
            (Param::Reg(Reg::L), Param::Reg(Reg::L)) => {
                cpu.reg.l = cpu.reg.l;
                4
            }
            (Param::Reg(Reg::L), Param::Addr(Addr::HL)) => {
                cpu.reg.l = Wrapping(cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            (Param::Addr(Addr::HL), Param::Reg(Reg::A)) => {
                cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.a.0);
                8
            }
            (Param::Addr(Addr::HL), Param::Reg(Reg::B)) => {
                cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.b.0);
                8
            }
            (Param::Addr(Addr::HL), Param::Reg(Reg::C)) => {
                cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.c.0);
                8
            }
            (Param::Addr(Addr::HL), Param::Reg(Reg::D)) => {
                cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.d.0);
                8
            }
            (Param::Addr(Addr::HL), Param::Reg(Reg::E)) => {
                cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.e.0);
                8
            }
            (Param::Addr(Addr::HL), Param::Reg(Reg::H)) => {
                cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.h.0);
                8
            }
            (Param::Addr(Addr::HL), Param::Reg(Reg::L)) => {
                cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.l.0);
                8
            }
            (Param::Addr(Addr::BC), Param::Reg(Reg::A)) => {
                cpu.mmu.write(cpu.reg.get_bc(), cpu.reg.a.0);
                8
            }
            (Param::Addr(Addr::DE), Param::Reg(Reg::A)) => {
                cpu.mmu.write(cpu.reg.get_bc(), cpu.reg.a.0);
                8
            }
            (Param::Addr(Addr::C), Param::Reg(Reg::A)) => {
                cpu.mmu.write(cpu.reg.c.0 as u16 | 0xff00, cpu.reg.a.0);
                8
            }
            (Param::Addr(Addr::HLInc), Param::Reg(Reg::A)) => {
                let hl = cpu.reg.get_hl();
                cpu.mmu.write(hl, cpu.reg.a.0);
                cpu.reg.set_hl(hl.wrapping_add(1));
                8
            }
            (Param::Addr(Addr::HLDec), Param::Reg(Reg::A)) => {
                let hl = cpu.reg.get_hl();
                cpu.mmu.write(hl, cpu.reg.a.0);
                cpu.reg.set_hl(hl.wrapping_sub(1));
                8
            }
            (Param::Addr(Addr::U8(n)), Param::Reg(Reg::A)) => {
                cpu.mmu.write(0xff00 | n as u16, cpu.reg.a.0);
                12
            }
            (Param::Addr(Addr::U16(n)), Param::Reg(Reg::A)) => {
                cpu.mmu.write(n, cpu.reg.a.0);
                16
            }
            (Param::Reg(Reg::BC), Param::Imm(Imm::U16(n))) => {
                cpu.reg.set_bc(n);
                12
            }
            (Param::Reg(Reg::DE), Param::Imm(Imm::U16(n))) => {
                cpu.reg.set_de(n);
                12
            }
            (Param::Reg(Reg::HL), Param::Imm(Imm::U16(n))) => {
                cpu.reg.set_hl(n);
                12
            }
            (Param::Reg(Reg::SP), Param::Imm(Imm::U16(n))) => {
                cpu.reg.sp.0 = n;
                12
            }
            (Param::Reg(Reg::SP), Param::Reg(Reg::HL)) => {
                cpu.reg.sp.0 = cpu.reg.get_hl();
                8
            }
            (Param::Reg(Reg::HL), Param::RegI8(RegI8::SP(n))) => {
                let hl = cpu.reg.alu_add_sp_i8(n);
                cpu.reg.set_hl(hl);
                12
            }
            (Param::Addr(Addr::U16(n)), Param::Reg(Reg::SP)) => {
                cpu.mmu.write_word(n, cpu.reg.sp.0);
                20
            }
            _ => unreachable!(),
        }
    }
}
