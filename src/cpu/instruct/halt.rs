use super::InstructionControl;
use crate::cpu::{Flags, CPU};

pub struct HALT;

impl InstructionControl for HALT {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        cpu.halted = true;
        4
    }
}
