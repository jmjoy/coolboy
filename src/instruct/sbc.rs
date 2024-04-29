use super::{Addr, Imm, InstructionControl, Param, Reg};
use crate::cpu::CPU;

pub struct SBC(pub(super) Param, pub(super) Param);

impl InstructionControl for SBC {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        match (self.0, self.1) {
            (Param::Reg(Reg::A), Param::Reg(Reg::A)) => {
                cpu.reg.a.0 = cpu.reg.alu_sbc(cpu.reg.a.0, cpu.reg.a.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::B)) => {
                cpu.reg.a.0 = cpu.reg.alu_sbc(cpu.reg.a.0, cpu.reg.b.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::C)) => {
                cpu.reg.a.0 = cpu.reg.alu_sbc(cpu.reg.a.0, cpu.reg.c.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::D)) => {
                cpu.reg.a.0 = cpu.reg.alu_sbc(cpu.reg.a.0, cpu.reg.d.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::E)) => {
                cpu.reg.a.0 = cpu.reg.alu_sbc(cpu.reg.a.0, cpu.reg.e.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::H)) => {
                cpu.reg.a.0 = cpu.reg.alu_sbc(cpu.reg.a.0, cpu.reg.h.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::L)) => {
                cpu.reg.a.0 = cpu.reg.alu_sbc(cpu.reg.a.0, cpu.reg.l.0);
                4
            }
            (Param::Reg(Reg::A), Param::Imm(Imm::U8(n))) => {
                cpu.reg.a.0 = cpu.reg.alu_sbc(cpu.reg.a.0, n);
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::HL)) => {
                cpu.reg.a.0 = cpu.reg.alu_sbc(cpu.reg.a.0, cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            _ => unreachable!(),
        }
    }
}
