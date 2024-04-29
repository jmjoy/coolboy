use super::InstructionControl;
use crate::cpu::CPU;

pub struct DAA;

impl InstructionControl for DAA {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        cpu.reg.a.0 = cpu.reg.alu_daa(cpu.reg.a.0);
        4
    }
}
