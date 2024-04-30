use super::InstructionControl;
use crate::cpu::{Flags, CPU};

pub struct CPL;

impl InstructionControl for CPL {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        cpu.reg.a = !cpu.reg.a ; 
        cpu.reg.f.insert(Flags::H);
        cpu.reg.f.insert(Flags::N);
        4
    }
}
