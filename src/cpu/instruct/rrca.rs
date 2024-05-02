use super::InstructionControl;
use crate::cpu::{Flags, CPU};

pub struct RRCA;

impl InstructionControl for RRCA {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        let c = cpu.reg.a & 0x01 == 0x01;
        let a = cpu.reg.a.rotate_right(1);
        cpu.reg.f.set(Flags::Z, a == 0);
        cpu.reg.f.remove(Flags::N);
        cpu.reg.f.remove(Flags::H);
        cpu.reg.f.set(Flags::C, c);
        cpu.reg.a = a;
        4
    }
}
