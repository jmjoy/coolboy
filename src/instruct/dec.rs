use super::{Addr, Imm, InstructionControl, Param, Reg};
use crate::cpu::CPU;

pub struct DEC(pub(super) Param);

impl InstructionControl for DEC {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        match self.0 {
            Param::Reg(Reg::A) => {
                cpu.reg.a.0 = cpu.reg.alu_dec(cpu.reg.a.0);
                4
            }
            Param::Reg(Reg::B) => {
                cpu.reg.b.0 = cpu.reg.alu_dec(cpu.reg.b.0);
                4
            }
            Param::Reg(Reg::C) => {
                cpu.reg.c.0 = cpu.reg.alu_dec(cpu.reg.c.0);
                4
            }
            Param::Reg(Reg::D) => {
                cpu.reg.d.0 = cpu.reg.alu_dec(cpu.reg.d.0);
                4
            }
            Param::Reg(Reg::E) => {
                cpu.reg.e.0 = cpu.reg.alu_dec(cpu.reg.e.0);
                4
            }
            Param::Reg(Reg::H) => {
                cpu.reg.h.0 = cpu.reg.alu_dec(cpu.reg.h.0);
                4
            }
            Param::Reg(Reg::L) => {
                cpu.reg.l.0 = cpu.reg.alu_dec(cpu.reg.l.0);
                4
            }
            Param::Addr(Addr::HL) => {
                let hl = cpu.reg.get_hl();
                cpu.mmu.write(hl, cpu.reg.alu_dec(cpu.mmu.read(hl)));
                12
            }
            Param::Reg(Reg::BC) => {
                let bc = cpu.reg.get_bc();
                cpu.reg.set_bc(bc.wrapping_sub(1));
                8
            }
            Param::Reg(Reg::DE) => {
                let de = cpu.reg.get_de();
                cpu.reg.set_de(de.wrapping_sub(1));
                8
            }
            Param::Reg(Reg::HL) => {
                let hl = cpu.reg.get_hl();
                cpu.reg.set_hl(hl.wrapping_sub(1));
                8
            }
            Param::Reg(Reg::SP) => {
                cpu.reg.sp -= 1;
                8
            }
            _ => unreachable!(),
        }
    }
}
