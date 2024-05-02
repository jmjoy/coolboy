use super::InstructionControl;
use crate::cpu::CPU;

pub struct STOP;

impl InstructionControl for STOP {
    #[inline]
    fn call(self, _cpu: &mut CPU) -> usize {
        4
    }
}
