use super::InstructionControl;
use crate::cpu::CPU;

pub struct DI;

impl InstructionControl for DI {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        cpu.di = true;
        4
    }
}
