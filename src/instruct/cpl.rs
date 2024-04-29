use super::InstructionControl;
use crate::cpu::CPU;

pub struct CPL;

impl InstructionControl for CPL {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        cpu.reg.alu_daa();
        4
    }
}
