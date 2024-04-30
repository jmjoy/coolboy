use super::{InstructionControl, Reg};
use crate::cpu::CPU;

pub struct POP(pub(super) Reg);

impl InstructionControl for POP {
    fn call(self, cpu: &mut CPU) -> usize {
        match self.0 {
            Reg::BC => {
                let val = cpu.pop_stack();
                cpu.reg.set_bc(val);
            }
            Reg::DE => {
                let val = cpu.pop_stack();
                cpu.reg.set_de(val);
            }
            Reg::HL => {
                let val = cpu.pop_stack();
                cpu.reg.set_hl(val);
            }
            Reg::AF => {
                let val = cpu.pop_stack();
                cpu.reg.set_af(val);
            }
            _ => unreachable!(),
        }
        20
    }
}
