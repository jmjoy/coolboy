use super::InstructionControl;
use crate::cpu::CPU;

pub struct EI;

impl InstructionControl for EI {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        cpu.ei = true;
        4
    }
}
