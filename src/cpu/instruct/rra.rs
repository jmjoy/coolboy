use super::InstructionControl;
use crate::cpu::{Flags, CPU};

pub struct RRA;

impl InstructionControl for RRA {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        let c = cpu.reg.a & 0x01 == 0x01;
        let mut a = cpu.reg.a >> 1;
        if cpu.reg.f.contains(Flags::C) {
            a |= 0x80;
        }
        cpu.reg.f.set(Flags::Z, a == 0);
        cpu.reg.f.remove(Flags::N);
        cpu.reg.f.remove(Flags::H);
        cpu.reg.f.set(Flags::C, c);
        cpu.reg.a = a;
        4
    }
}
