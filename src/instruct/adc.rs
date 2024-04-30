use super::{Addr, Imm, InstructionControl, Param, Reg};
use crate::cpu::CPU;

pub struct ADC(pub Param, pub Param);

impl InstructionControl for ADC {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        match (self.0, self.1) {
            (Param::Reg(Reg::A), Param::Reg(Reg::A)) => {
                cpu.reg.a = cpu.reg.alu_adc(cpu.reg.a, cpu.reg.a);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::B)) => {
                cpu.reg.a = cpu.reg.alu_adc(cpu.reg.a, cpu.reg.b);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::C)) => {
                cpu.reg.a = cpu.reg.alu_adc(cpu.reg.a, cpu.reg.c);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::D)) => {
                cpu.reg.a = cpu.reg.alu_adc(cpu.reg.a, cpu.reg.d);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::E)) => {
                cpu.reg.a = cpu.reg.alu_adc(cpu.reg.a, cpu.reg.e);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::H)) => {
                cpu.reg.a = cpu.reg.alu_adc(cpu.reg.a, cpu.reg.h);
                4
            }
            (Param::Reg(Reg::A), Param::Reg(Reg::L)) => {
                cpu.reg.a = cpu.reg.alu_adc(cpu.reg.a, cpu.reg.l);
                4
            }
            (Param::Reg(Reg::A), Param::Imm(Imm::U8(n))) => {
                cpu.reg.a = cpu.reg.alu_adc(cpu.reg.a, n);
                8
            }
            (Param::Reg(Reg::A), Param::Addr(Addr::HL)) => {
                cpu.reg.a = cpu.reg.alu_adc(cpu.reg.a, cpu.mmu.read(cpu.reg.get_hl()));
                8
            }
            _ => unreachable!(),
        }
    }
}
