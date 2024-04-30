use super::InstructionControl;
use crate::cpu::CPU;

pub struct NOP;

impl InstructionControl for NOP {
    #[inline]
    fn call(self, _cpu: &mut CPU) -> usize {
        1
    }
}
