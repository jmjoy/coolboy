use super::{Cond, InstructionControl};
use crate::cpu::{Flags, CPU};

pub(super) struct JR(pub(super) Option<Cond>, pub(super) u8);

impl InstructionControl for JR {
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
                    pc_add_signed(cpu, self.1);
                    12
                } else {
                    8
                }
            }
            None => 12,
        }
    }
}

fn pc_add_signed(cpu: &mut CPU, u: u8) {
    let (pc, _) = cpu.reg.pc.0.overflowing_add_signed(u as i16);
    cpu.reg.pc.0 = pc;
}
