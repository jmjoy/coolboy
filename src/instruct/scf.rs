use super::InstructionControl;
use crate::cpu::{Flags, CPU};

pub struct SCF;

impl InstructionControl for SCF {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        cpu.reg.f.insert(Flags::C);
        cpu.reg.f.remove(Flags::H);
        cpu.reg.f.remove(Flags::N);
        4
    }
}
