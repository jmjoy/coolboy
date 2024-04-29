use super::{Addr, Imm, InstructionControl, Param, Reg};
use crate::cpu::CPU;

pub struct CP(pub Param, pub Param);

impl InstructionControl for CP {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        match (self.0, self.1) {
            (Param::Reg(Reg::A), Param::Reg(Reg::A)) => {
                cpu.reg.alu_sub(cpu.reg.a.0, cpu.reg.a.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::B)) => {
                cpu.reg.alu_sub(cpu.reg.a.0, cpu.reg.b.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::C)) => {
                cpu.reg.alu_sub(cpu.reg.a.0, cpu.reg.c.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::D)) => {
                cpu.reg.alu_sub(cpu.reg.a.0, cpu.reg.d.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::E)) => {
                cpu.reg.alu_sub(cpu.reg.a.0, cpu.reg.e.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::H)) => {
                cpu.reg.alu_sub(cpu.reg.a.0, cpu.reg.h.0);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::L)) => {
                cpu.reg.alu_sub(cpu.reg.a.0, cpu.reg.l.0);
                4
            }
            (Param::Reg(Reg::A), Param::Imm(Imm::U8(n))) => {
                cpu.reg.alu_sub(cpu.reg.a.0, n);
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::HL)) => {
                cpu.reg.alu_sub(cpu.reg.a.0, cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            _ => unreachable!(),
        }
    }
}
