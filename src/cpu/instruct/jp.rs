use super::{Cond, InstructionControl};
use crate::cpu::{Flags, CPU};

pub(super) struct JP(pub(super) Option<Cond>, pub(super) u16);

impl InstructionControl for JP {
    #[inline]
    fn call(self, cpu: &mut CPU) -> usize {
        match self.0 {
            Some(cond) => {
                let ok = match cond {
                    Cond::Z => cpu.reg.f.contains(Flags::Z),
                    Cond::NZ => !cpu.reg.f.contains(Flags::Z),
                    Cond::C => cpu.reg.f.contains(Flags::C),
                    Cond::NC => !cpu.reg.f.contains(Flags::C),
                };
                if ok {
                    cpu.reg.pc.0 = self.1;
                    16
                } else {
                    12
                }
            }
            None => {
                cpu.reg.pc.0 = self.1;
                16
            }
        }
    }
}
