use super::InstructionControl;
use crate::cpu::{Flags, CPU};

pub struct CCF;

impl InstructionControl for CCF {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        cpu.reg.f.toggle(Flags::C);
        cpu.reg.f.remove(Flags::H);
        cpu.reg.f.remove(Flags::N);
        4
    }
}
