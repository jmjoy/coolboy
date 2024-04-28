use super::InstructionControl;
use crate::cpu::CPU;

pub struct NOP;

impl InstructionControl for NOP {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        1
    }
}
