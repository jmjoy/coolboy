use super::{Addr, Imm, InstructionControl, Param, Reg};
use crate::cpu::CPU;

pub struct CP(pub Param, pub Param);

impl InstructionControl for CP {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        match (self.0, self.1) {
            (Param::Reg(Reg::A), Param::Reg(Reg::A)) => {
                cpu.reg.alu_sub(cpu.reg.a, cpu.reg.a);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::B)) => {
                cpu.reg.alu_sub(cpu.reg.a, cpu.reg.b);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::C)) => {
                cpu.reg.alu_sub(cpu.reg.a, cpu.reg.c);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::D)) => {
                cpu.reg.alu_sub(cpu.reg.a, cpu.reg.d);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::E)) => {
                cpu.reg.alu_sub(cpu.reg.a, cpu.reg.e);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::H)) => {
                cpu.reg.alu_sub(cpu.reg.a, cpu.reg.h);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::L)) => {
                cpu.reg.alu_sub(cpu.reg.a, cpu.reg.l);
                4
            }
            (Param::Reg(Reg::A), Param::Imm(Imm::U8(n))) => {
                cpu.reg.alu_sub(cpu.reg.a, n);
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::HL)) => {
                cpu.reg.alu_sub(cpu.reg.a, cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            _ => unreachable!(),
        }
    }
}
