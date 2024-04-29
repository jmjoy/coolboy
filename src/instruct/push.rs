use super::{InstructionControl, Reg};
use crate::cpu::CPU;

pub struct PUSH(pub Reg);

impl InstructionControl for PUSH {
    fn call(self, cpu: &mut CPU) -> usize {
        match self.0 {
            Reg::BC => cpu.push_stack(cpu.reg.get_bc()),
            Reg::DE => cpu.push_stack(cpu.reg.get_de()),
            Reg::HL => cpu.push_stack(cpu.reg.get_hl()),
            Reg::AF => cpu.push_stack(cpu.reg.get_af()),
            _ => unreachable!(),
        }
        20
    }
}
