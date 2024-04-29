use super::{Addr, Imm, InstructionControl, Param, Reg};
use crate::cpu::CPU;

pub struct ADD(pub(super) Param, pub(super) Param);

impl InstructionControl for ADD {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        match (self.0, self.1) {
            (Param::Reg(Reg::A), Param::Reg(Reg::A)) => {
                cpu.reg.a.0 = cpu.reg.alu_add(cpu.reg.a.0, cpu.reg.a.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::B)) => {
                cpu.reg.a.0 = cpu.reg.alu_add(cpu.reg.a.0, cpu.reg.b.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::C)) => {
                cpu.reg.a.0 = cpu.reg.alu_add(cpu.reg.a.0, cpu.reg.c.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::D)) => {
                cpu.reg.a.0 = cpu.reg.alu_add(cpu.reg.a.0, cpu.reg.d.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::E)) => {
                cpu.reg.a.0 = cpu.reg.alu_add(cpu.reg.a.0, cpu.reg.e.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::H)) => {
                cpu.reg.a.0 = cpu.reg.alu_add(cpu.reg.a.0, cpu.reg.h.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::L)) => {
                cpu.reg.a.0 = cpu.reg.alu_add(cpu.reg.a.0, cpu.reg.l.0);
                4
            }
            (Param::Reg(Reg::A), Param::Imm(Imm::U8(n))) => {
                cpu.reg.a.0 = cpu.reg.alu_add(cpu.reg.a.0, n);
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::HL)) => {
                cpu.reg.a.0 = cpu.reg.alu_add(cpu.reg.a.0, cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            (Param::Reg(Reg::HL), Param::Reg(Reg::BC)) => {
                let hl = cpu.reg.get_hl();
                let val = cpu.reg.get_bc();
                let val = cpu.reg.alu_add_u16(hl, val);
                cpu.reg.set_hl(val);
                8
            }
            (Param::Reg(Reg::HL), Param::Reg(Reg::DE)) => {
                let hl = cpu.reg.get_hl();
                let val = cpu.reg.get_de();
                let val = cpu.reg.alu_add_u16(hl, val);
                cpu.reg.set_hl(val);
                8
            }
            (Param::Reg(Reg::HL), Param::Reg(Reg::HL)) => {
                let hl = cpu.reg.get_hl();
                let val = cpu.reg.alu_add_u16(hl, hl);
                cpu.reg.set_hl(val);
                8
            }
            (Param::Reg(Reg::HL), Param::Reg(Reg::SP)) => {
                let hl = cpu.reg.get_hl();
                let val = cpu.reg.sp.0;
                let val = cpu.reg.alu_add_u16(hl, val);
                cpu.reg.set_hl(val);
                8
            }
            (Param::Reg(Reg::SP), Param::Imm(Imm::U8(n))) => {
                cpu.reg.sp.0 = cpu.reg.alu_add_sp_i8(n);
                4
            }
            _ => unreachable!(),
        }
    }
}
