use super::InstructionControl;
use crate::cpu::{Flags, CPU};

pub struct RLCA;

impl InstructionControl for RLCA {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        let c = cpu.reg.a & 0x80 == 0x80;
        let a = cpu.reg.a.rotate_left(1);
        cpu.reg.f.set(Flags::Z, a == 0);
        cpu.reg.f.remove(Flags::N);
        cpu.reg.f.remove(Flags::H);
        cpu.reg.f.set(Flags::C, c);
        cpu.reg.a = a;
        4
    }
}
